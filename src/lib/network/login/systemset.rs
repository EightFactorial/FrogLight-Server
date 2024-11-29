use bevy::prelude::SystemSet;

/// A [`SystemSet`] for managing login connections.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct LoginSystemSet;

/// A [`SystemSet`] for receiving login packets.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct LoginPacketSet;
