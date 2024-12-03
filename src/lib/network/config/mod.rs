//! TODO

use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

mod task;

mod types;
pub use types::*;

mod version;
pub use version::ConfigTrait;

use super::login::LoginStateEvent;

/// A [`Plugin`] that receives logged in and reconfiguring clients.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfigPlugin<V: Version>(PhantomData<V>);

impl<V: Version + ConfigTrait> Plugin for ConfigPlugin<V>
where
    Clientbound: NetworkDirection<V, Login> + NetworkDirection<V, Configuration>,
    Login: State<V>,
    Configuration: State<V>,
{
    fn build(&self, app: &mut App) {
        // Add events and initialize resources
        app.add_event::<ConfigStateEvent<V>>();
        app.add_event::<ConfigPacketEvent<V>>();
        app.init_resource::<ConfigFilter<V>>();
        app.init_resource::<ConfigRequiredComponents<V>>();

        // Add systems
        app.add_systems(
            PreUpdate,
            ConfigTask::<V>::receive_packets.run_if(any_with_component::<ConfigTask<V>>),
        );
        app.add_systems(
            Update,
            ConfigTask::<V>::complete_configurations.run_if(any_with_component::<ConfigTask<V>>),
        );
        app.add_systems(
            PostUpdate,
            (
                ConfigTask::<V>::receive_logins.run_if(on_event::<LoginStateEvent<V>>),
                ConfigTask::<V>::poll_tasks.run_if(any_with_component::<ConfigTask<V>>),
            ),
        );
    }
}
