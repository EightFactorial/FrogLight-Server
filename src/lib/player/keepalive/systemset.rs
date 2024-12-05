use bevy::prelude::SystemSet;

/// A [`SystemSet`] that for systems that manaage keep-alive packets.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct KeepAliveSystemSet;
