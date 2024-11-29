use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

mod systemset;
pub use systemset::*;

mod task;

mod types;
pub use types::*;

mod version;
pub use version::PlayTrait;

use super::{ConfigPacketSet, ConfigStateEvent, ConfigSystemSet};

static TARGET: &str = "PLAY";

/// A plugin for managing [`Connection`]s in the [`Play`] state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayPlugin<V: Version> {
    _phantom: PhantomData<V>,
}

impl<V: Version + PlayTrait> Plugin for PlayPlugin<V>
where
    Clientbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    fn build(&self, app: &mut App) {
        // Only configure systemsets if they doesn't already exist
        if app
            .get_schedule(PostUpdate)
            .is_some_and(|sched| !sched.graph().contains_set(PlayPacketSet))
        {
            app.configure_sets(PreUpdate, PlayPacketSet.after(ConfigPacketSet));
            app.configure_sets(PostUpdate, PlaySystemSet.after(ConfigSystemSet));
        }

        app.add_event::<PlayPacketEvent<V>>();
        app.add_event::<PlayStateEvent<V>>();
        app.init_resource::<PlayFilter<V>>();

        app.add_systems(
            PreUpdate,
            PlayTask::<V>::poll_packets
                .run_if(any_with_component::<PlayTask<V>>)
                .in_set(PlayPacketSet),
        );
        app.add_systems(
            PostUpdate,
            (
                PlayTask::<V>::receive_configured
                    .run_if(on_event::<ConfigStateEvent<V>>)
                    .in_set(PlaySystemSet),
                PlayTask::<V>::poll_tasks
                    .run_if(any_with_component::<PlayTask<V>>)
                    .in_set(PlaySystemSet),
            ),
        );
    }
}
