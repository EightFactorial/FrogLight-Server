//! TODO

use bevy::prelude::*;
use froglight::{network::connection::NetworkDirection, prelude::*};

mod version;
pub use version::PlayerInitialize;

use crate::dimension::{All, DimensionApp};

#[doc(hidden)]
pub(super) fn build<V: Version + PlayerInitialize>(app: &mut App)
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    app.in_dimension(All, |app| {
        app.add_systems(PreUpdate, initialize_new_players);
    });
}

/// A system that initializes new players.
pub fn initialize_new_players<V: Version + PlayerInitialize>(
    query: Query<(Entity, &GameProfile), Added<GameProfile>>,
    world: &World,
    mut commands: Commands,
) where
    Play: State<V>,
{
    for (entity, profile) in &query {
        debug!("Initializing {}", profile.name);
        for event in V::initialize(entity, world) {
            commands.send_event(event);
        }
    }
}
