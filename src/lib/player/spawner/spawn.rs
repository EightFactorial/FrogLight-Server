//! TODO

use std::hash::BuildHasherDefault;

use bevy::utils::HashMap;
use froglight::prelude::*;

use crate::dimension::{subapp::DimensionIdentifier, Overworld};

/// A struct that manages player spawnpoints.
#[derive(Debug)]
pub struct PlayerSpawner {
    /// The default dimension and position to spawn new players at.
    pub default: PlayerSpawnerPosition,
    /// The dimension and position to spawn a specific player at.
    pub player: HashMap<Uuid, PlayerSpawnerPosition>,
}

impl PlayerSpawner {
    pub(super) const DEFAULT_POS: BlockPosition = BlockPosition::new(0, 128, 0);
    pub(super) const DEFAULT_DIM: Overworld = Overworld;

    /// Create a new [`PlayerSpawner`] using the internal default values.
    #[must_use]
    pub(super) fn internal_default() -> Self {
        Self::new(Self::DEFAULT_POS, Self::DEFAULT_DIM.into())
    }

    /// Create a new [`PlayerSpawner`] with the default spawn point.
    #[must_use]
    pub const fn new(position: BlockPosition, dimension: DimensionIdentifier) -> Self {
        Self {
            default: PlayerSpawnerPosition { dimension, position },
            player: HashMap::with_hasher(BuildHasherDefault::new()),
        }
    }
}

impl PlayerSpawner {
    /// Get the default player spawn point.
    #[must_use]
    pub fn default(&self) -> &PlayerSpawnerPosition { &self.default }

    /// Returns `true` if the player's spawn point is set.
    #[must_use]
    pub fn contains(&self, uuid: &Uuid) -> bool { self.player.contains_key(uuid) }

    /// Get a reference to the player's spawn point.
    #[must_use]
    pub fn get(&self, uuid: &Uuid) -> Option<&PlayerSpawnerPosition> { self.player.get(uuid) }

    /// Get a mutable reference to the player's spawn point.
    #[must_use]
    pub fn get_mut(&mut self, uuid: &Uuid) -> Option<&mut PlayerSpawnerPosition> {
        self.player.get_mut(uuid)
    }

    /// Get the player's spawn point, or the default if none is set.
    #[must_use]
    pub fn get_or_default(&self, uuid: &Uuid) -> &PlayerSpawnerPosition {
        self.get(uuid).unwrap_or(&self.default)
    }

    /// Get a mutable reference to the player's spawn point,
    /// setting it to the default if none is set.
    pub fn get_or_set_default(&mut self, uuid: Uuid) -> &mut PlayerSpawnerPosition {
        self.player.entry(uuid).or_insert(self.default)
    }

    /// Set the dimension and position to spawn a player at.
    ///
    /// Returns the previous value, if any.
    pub fn set(
        &mut self,
        uuid: Uuid,
        position: BlockPosition,
        dimension: DimensionIdentifier,
    ) -> Option<PlayerSpawnerPosition> {
        self.player.insert(uuid, PlayerSpawnerPosition { dimension, position })
    }
}

/// A dimension and position to spawn a player at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerSpawnerPosition {
    /// The dimension to spawn the player in.
    pub dimension: DimensionIdentifier,
    /// The position to spawn the player at.
    pub position: BlockPosition,
}
