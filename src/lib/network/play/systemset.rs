use bevy::prelude::SystemSet;

/// A [`SystemSet`] for managing play connections.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct PlaySystemSet;

/// A [`SystemSet`] for receiving play packets.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct PlayPacketSet;
