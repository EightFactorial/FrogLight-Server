use bevy::prelude::{Commands, Entity, World};
use froglight::prelude::Version;

mod v1_21_0;

/// A trait for initializing players.
pub trait InitializeTrait: Version {
    /// Initialize a player entity.
    fn initialize(entity: Entity, world: &World, commands: &mut Commands);
}
