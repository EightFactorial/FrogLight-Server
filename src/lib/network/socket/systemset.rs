use bevy::prelude::SystemSet;

/// A [`SystemSet`] for listening for incoming connections.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct ListenSystemSet;
