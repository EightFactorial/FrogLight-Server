use bevy::prelude::SystemSet;

/// A [`SystemSet`] that for network systems.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct NetworkExtSystemSet;
