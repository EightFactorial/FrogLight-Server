use bevy::prelude::SystemSet;

/// A [`SystemSet`] that for systems that manage keep-alive packets.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct KeepAliveSystemSet;
