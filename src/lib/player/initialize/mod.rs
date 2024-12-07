//! TODO

use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{network::connection::NetworkDirection, prelude::*};

mod version;
pub use version::InitializeTrait;

use crate::dimension::{subapp::MainAppMarker, All, DimensionApp, Network};

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
                Network,
                initialize_player_connection::<V>
                    .run_if(any_with_component::<MainAppMarker>)
                    .ambiguous_with_all(),
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
#[expect(clippy::type_complexity)]
fn initialize_player_connection<V: Version + InitializeTrait>(
    query: Query<(Entity, &GameProfile), (Added<MainAppMarker>, Without<HasJoinPacket>)>,
    world: &World,
    mut commands: Commands,
) {
    for (entity, profile) in &query {
        debug!("Initializing player {}", profile.username);
        V::initialize(entity, world, &mut commands);
    }
}
