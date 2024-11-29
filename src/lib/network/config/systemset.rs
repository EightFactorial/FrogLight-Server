use bevy::prelude::SystemSet;

/// A [`SystemSet`] for managing config connections.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct ConfigSystemSet;

/// A [`SystemSet`] for receiving config packets.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct ConfigPacketSet;
