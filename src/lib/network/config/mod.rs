//! TODO

use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

mod registry;
pub use registry::ConfigRegistryTrait;

mod task;

mod types;
pub use types::*;

mod version;
pub use version::ConfigTrait;

use super::login::LoginStateEvent;

/// A [`Plugin`] that receives logged in and reconfiguring clients.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfigPlugin<V: Version>(PhantomData<V>);

impl<V: Version + ConfigTrait + ConfigRegistryTrait> Plugin for ConfigPlugin<V>
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

        // Add `HasRegistries` as a required config component
        let mut required = ConfigRequiredComponents::<V>::default();
        required.add_required::<HasRegistries>();
        app.insert_resource(required);

        // Add systems
        app.add_systems(
            PreUpdate,
            ConfigTask::<V>::receive_packets
                .run_if(any_with_component::<ConfigTask<V>>)
                .ambiguous_with_all(),
        );
        app.add_systems(
            Update,
            (ConfigTask::<V>::complete_configurations, ConfigTask::<V>::send_registries)
                .run_if(any_with_component::<ConfigTask<V>>)
                .ambiguous_with_all(),
        );
        app.add_systems(
            PostUpdate,
            (
                ConfigTask::<V>::receive_logins.run_if(on_event::<LoginStateEvent<V>>),
                ConfigTask::<V>::poll_tasks.run_if(any_with_component::<ConfigTask<V>>),
            )
                .ambiguous_with_all(),
        );
    }
}
