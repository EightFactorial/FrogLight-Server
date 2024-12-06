use bevy::prelude::SystemSet;

/// A [`SystemSet`] that for systems that manage player movement and position.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct MovementSystemSet;
