//! TODO

use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{network::connection::NetworkDirection, prelude::*};

mod version;
pub use version::InitializeTrait;

use crate::dimension::{subapp::MainAppMarker, All, DimensionApp};

/// A [`Plugin`] that initializes player connections.
#[derive(Debug, Default)]
pub struct PlayerInitializePlugin<V: Version>(PhantomData<V>);

impl<V: Version + InitializeTrait> Plugin for PlayerInitializePlugin<V>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    fn build(&self, app: &mut App) {
        app.register_type::<HasJoinPacket>();

        app.in_dimension(All, |app| {
            app.add_systems(
                PreUpdate,
                initialize_player_connection::<V>.run_if(any_with_component::<MainAppMarker>),
            );
        });
    }
}

/// A marker [`Component`] for connections that have the join packet.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[component(storage = "SparseSet")]
#[reflect(Component)]
pub struct HasJoinPacket;

/// A system that initializes new player connections.
fn initialize_player_connection<V: Version + InitializeTrait>(
    query: Query<Entity, Added<MainAppMarker>>,
    world: &World,
    mut commands: Commands,
) {
    for entity in &query {
        V::initialize(entity, world, &mut commands);
    }
}