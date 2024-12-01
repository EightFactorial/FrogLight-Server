use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use crate::{
    network::PlayTask,
    network_ext::{NetworkExtPlaySet, TARGET},
};

mod v1_21_0;

/// A [`Plugin`] that spawns newly connected clients.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlaySpawnPlugin<V: Version>(PhantomData<V>);

impl<V: Version + PlaySpawnTrait> Plugin for PlaySpawnPlugin<V>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::spawn_client.in_set(NetworkExtPlaySet));
    }
}

impl<V: Version + PlaySpawnTrait> PlaySpawnPlugin<V>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    /// A system that spawns newly connected clients.
    #[expect(clippy::type_complexity)]
    pub fn spawn_client(
        query: Query<(Entity, &GameProfile, &PlayTask<V>), Added<PlayTask<V>>>,
        mut commands: Commands,
    ) {
        for (entity, profile, task) in &query {
            debug!(target: TARGET, "Sending position to {}", profile.name);
            V::spawn_client(task, &mut commands.entity(entity));
        }
    }
}

/// A trait that spawns newly connected clients.
pub trait PlaySpawnTrait: Version
where
    Clientbound: NetworkDirection<Self, Play>,
    Play: State<Self>,
{
    /// Spawns a newly connected client.
    fn spawn_client(task: &PlayTask<Self>, commands: &mut EntityCommands);
}
