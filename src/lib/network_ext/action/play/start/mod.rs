use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use crate::{
    network::PlayTask,
    network_ext::{NetworkExtPlaySet, TARGET},
    world::{DimensionList, EntityIds},
};

mod v1_21_0;

/// A [`Plugin`] that initializes newly connected clients.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayStartPlugin<V: Version>(PhantomData<V>);

impl<V: Version + PlayStartTrait> Plugin for PlayStartPlugin<V>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::initialize_client.in_set(NetworkExtPlaySet));
    }
}

impl<V: Version + PlayStartTrait> PlayStartPlugin<V>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    /// A system that initializes newly connected clients.
    #[expect(clippy::type_complexity)]
    pub fn initialize_client(
        query: Query<(Entity, &GameProfile, &PlayTask<V>), Added<PlayTask<V>>>,
        entity_ids: Res<EntityIds>,
        dimensions: Res<DimensionList>,
        mut commands: Commands,
    ) {
        for (entity, profile, task) in query.iter() {
            let entity_id = entity_ids.get();
            debug!(target: TARGET, "Assigning EntityId {entity_id} to {}", profile.name);

            V::initialize_client(entity_id, &dimensions, task);
            commands.entity(entity).insert(entity_id);
        }
    }
}

/// A trait that initializes newly connected clients.
pub trait PlayStartTrait: Version
where
    Clientbound: NetworkDirection<Self, Play>,
    Play: State<Self>,
{
    /// Initialize a newly connected client.
    fn initialize_client(entity: EntityId, dimensions: &DimensionList, task: &PlayTask<Self>);
}
