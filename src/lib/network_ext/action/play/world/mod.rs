use std::marker::PhantomData;

use bevy::prelude::*;
use derive_more::derive::Deref;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use crate::{
    network::PlayTask,
    network_ext::{NetworkExtPlaySet, TARGET},
};

mod v1_21_0;

/// A [`Plugin`] that sends chunks to connected clients.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayChunkPlugin<V: Version>(PhantomData<V>);

/// A [`Component`] that stores a player's previous chunk position.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Component)]
pub struct PreviousChunk(ChunkPosition);

impl<V: Version + PlayChunkTrait> Plugin for PlayChunkPlugin<V>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (Self::send_initial_chunks, Self::send_new_chunks).in_set(NetworkExtPlaySet),
        );
    }
}

impl<V: Version + PlayChunkTrait> PlayChunkPlugin<V>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    /// A system that sends initial chunks to connected clients.
    pub fn send_initial_chunks(
        clients: Query<(&ChunkPosition, &GameProfile, &PlayTask<V>), Added<ChunkPosition>>,
        chunks: Query<&Chunk>,
        chunkmap: Res<ChunkPositionMap>,
        _commands: Commands,
    ) {
        for (pos, profile, task) in &clients {
            let entities = pos
                .in_radius::<5>()
                .into_iter()
                .filter_map(|pos| chunkmap.get(&pos).map(|e| (pos, *e)));
            let chunks: Vec<_> =
                entities.filter_map(|(pos, e)| chunks.get(e).ok().map(|c| (pos, c))).collect();

            debug!(target: TARGET, "Sending {} initial chunks to {}", chunks.len(), profile.name);
            V::send_chunks(&chunks, task);
        }
    }

    /// A system that sends new chunks to connected clients.
    pub fn send_new_chunks(
        mut query: Query<
            (&ChunkPosition, &mut PreviousChunk, &GameProfile, &PlayTask<V>),
            Changed<ChunkPosition>,
        >,
    ) {
        for (pos, mut prev, profile, task) in &mut query {
            debug!(target: TARGET, "Sending new chunks to {}", profile.name);
            V::send_chunks(&[], task);
            *prev = PreviousChunk(*pos);
        }
    }
}

/// A trait that sends chunks to connected clients.
pub trait PlayChunkTrait: Version
where
    Clientbound: NetworkDirection<Self, Play>,
    Play: State<Self>,
{
    /// Sends initial chunks to a connected client.
    fn send_chunks(chunks: &[(ChunkPosition, &Chunk)], task: &PlayTask<Self>);
}
