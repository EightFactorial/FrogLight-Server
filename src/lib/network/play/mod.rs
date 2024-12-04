//! TODO

use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{network::connection::NetworkDirection, prelude::*};

use crate::dimension::{All, DimensionApp, Network};

mod version;
pub use version::PlayTrait;

mod task;

mod types;
pub use types::*;

use super::config::ConfigStateEvent;

/// A [`Plugin`] that receives configured clients.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayPlugin<V: Version>(PhantomData<V>);

impl<V: Version + PlayTrait> Plugin for PlayPlugin<V>
where
    Clientbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    fn build(&self, app: &mut App) {
        // Add events and initialize resources
        app.add_event::<PlayStateEvent<V>>();
        app.init_resource::<PlayFilter<V>>();

        // Add dimension events and resources
        app.add_dimension_event::<PlayClientPacketEvent<V>>(All);
        app.add_dimension_event::<PlayServerPacketEvent<V>>(All);
        app.init_dimension_resource::<PlayRequiredComponents<V>>(All);

        // Add systems
        app.add_systems(
            PreUpdate,
            PlayTask::<V>::app_queue_and_receive_packets.run_if(any_with_component::<PlayTask<V>>),
        );
        app.add_systems(
            Update,
            PlayTask::<V>::reconfigure_session.run_if(any_with_component::<PlayTask<V>>),
        );
        app.add_systems(
            PostUpdate,
            (
                PlayTask::<V>::receive_configured.run_if(on_event::<ConfigStateEvent<V>>),
                PlayTask::<V>::poll_tasks.run_if(any_with_component::<PlayTask<V>>),
            ),
        );

        // Initialize and insert the shared event queue
        let queue = PlayPacketEventQueue::<V>::default();
        app.insert_dimension_resource(All, queue.clone());
        app.insert_resource(queue);

        // Add dimension systems
        app.add_dimension_systems(All, Network, PlayTask::<V>::sub_queue_and_receive_packets);
    }
}
