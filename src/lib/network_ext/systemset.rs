use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{network::connection::NetworkDirection, prelude::*};

use crate::network::{ConfigTask, LoginTask, PlayTask};

/// A [`Plugin`] for adding and configuring [`SystemSet`]s.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SystemSetPlugin<V: Version>(PhantomData<V>);

impl<V: Version> Plugin for SystemSetPlugin<V>
where
    Clientbound:
        NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    fn build(&self, app: &mut App) {
        // Only configure systemsets if they doesn't already exist
        if app
            .get_schedule(PostUpdate)
            .is_some_and(|sched| !sched.graph().contains_set(NetworkExtSystemSet))
        {
            app.configure_sets(Update, NetworkExtSystemSet);

            app.configure_sets(
                Update,
                NetworkExtLoginSet
                    .in_set(NetworkExtSystemSet)
                    .run_if(any_with_component::<LoginTask<V>>),
            );
            app.configure_sets(
                Update,
                NetworkExtConfigSet
                    .in_set(NetworkExtSystemSet)
                    .run_if(any_with_component::<ConfigTask<V>>),
            );
            app.configure_sets(
                Update,
                NetworkExtPlaySet
                    .in_set(NetworkExtSystemSet)
                    .run_if(any_with_component::<PlayTask<V>>),
            );
        }
    }
}

/// A [`SystemSet`] that for network systems.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct NetworkExtSystemSet;

/// A [`SystemSet`] for the login phase.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct NetworkExtLoginSet;

/// A [`SystemSet`] for the config phase.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct NetworkExtConfigSet;

/// A [`SystemSet`] for the play phase.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct NetworkExtPlaySet;
