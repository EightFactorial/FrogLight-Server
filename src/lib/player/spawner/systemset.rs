use bevy::prelude::SystemSet;

/// A [`SystemSet`] that for systems that manage initial player spawns.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct SpawnerSystemSet;
