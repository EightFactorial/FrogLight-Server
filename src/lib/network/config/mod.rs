use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

mod systemset;
pub use systemset::*;

mod types;
pub use types::*;

mod task;

mod version;
pub use version::ConfigTrait;

use super::LoginStateEvent;

static TARGET: &str = "CONF";

/// A plugin for managing [`Connection`]s in the [`Configuration`] state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfigPlugin<V: Version> {
    _phantom: PhantomData<V>,
}

impl<V: Version + ConfigTrait> Plugin for ConfigPlugin<V>
where
    Clientbound: NetworkDirection<V, Login> + NetworkDirection<V, Configuration>,
    Login: State<V>,
    Configuration: State<V>,
{
    fn build(&self, app: &mut App) {
        // Only configure systemsets if they doesn't already exist
        if app
            .get_schedule(PostUpdate)
            .is_some_and(|sched| !sched.graph().contains_set(ConfigSystemSet))
        {
            app.configure_sets(PreUpdate, ConfigPacketSet);
            app.configure_sets(PostUpdate, ConfigSystemSet);
        }

        app.add_event::<ConfigPacketEvent<V>>();
        app.add_event::<ConfigStateEvent<V>>();
        app.init_resource::<ConfigFilter<V>>();

        app.add_systems(
            PreUpdate,
            ConfigTask::<V>::poll_packets
                .run_if(any_with_component::<ConfigTask<V>>)
                .in_set(ConfigPacketSet),
        );
        app.add_systems(
            PostUpdate,
            (
                ConfigTask::<V>::receive_logins
                    .run_if(on_event::<LoginStateEvent<V>>)
                    .in_set(ConfigSystemSet),
                ConfigTask::<V>::poll_tasks
                    .run_if(any_with_component::<ConfigTask<V>>)
                    .in_set(ConfigSystemSet),
            ),
        );
    }
}
