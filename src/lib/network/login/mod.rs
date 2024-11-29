use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

mod info;
pub use info::ConnectionInfo;

mod systemset;
pub use systemset::{LoginPacketSet, LoginSystemSet};

mod types;
pub use types::*;

mod task;

mod version;
pub use version::LoginTrait;

use super::ConnectionRequestEvent;

static TARGET: &str = "LOGN";

/// A plugin for managing [`Connection`]s in the [`Login`] state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoginPlugin<V: Version> {
    _phantom: PhantomData<V>,
}

impl<V: Version + LoginTrait> Plugin for LoginPlugin<V>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    fn build(&self, app: &mut App) {
        // Only configure systemsets if they doesn't already exist
        if app
            .get_schedule(PostUpdate)
            .is_some_and(|sched| !sched.graph().contains_set(LoginSystemSet))
        {
            app.configure_sets(PreUpdate, LoginPacketSet);
            app.configure_sets(PostUpdate, LoginSystemSet);
        }

        app.add_event::<LoginPacketEvent<V>>();
        app.add_event::<LoginStateEvent<V>>();
        app.init_resource::<LoginFilter<V>>();

        app.add_systems(
            PreUpdate,
            LoginTask::<V>::poll_packets
                .run_if(any_with_component::<LoginTask<V>>)
                .in_set(LoginPacketSet),
        );
        app.add_systems(
            PostUpdate,
            (
                LoginTask::<V>::receive_requests.run_if(on_event::<ConnectionRequestEvent<V>>),
                LoginTask::<V>::poll_tasks.run_if(any_with_component::<LoginTask<V>>),
            )
                .in_set(LoginSystemSet),
        );
    }
}
