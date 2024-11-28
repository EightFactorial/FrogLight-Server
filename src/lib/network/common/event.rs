use std::sync::Arc;

use bevy::prelude::{Entity, Event};
use froglight::{network::connection::NetworkDirection, prelude::*};
use parking_lot::Mutex;

/// An [`Event`] that is sent when a [`Connection`] changes [`States`](State).
#[derive(Event)]
pub struct ConnectionStateEvent<V: Version, S: State<V>>
where
    Clientbound: NetworkDirection<V, S>,
{
    /// The [`Entity`] the [`Connection`] is associated with.
    pub entity: Entity,
    /// The [`Connection`].
    pub connection: Mutex<Option<Connection<V, S, Clientbound>>>,
}

impl<V: Version, S: State<V>> ConnectionStateEvent<V, S>
where
    Clientbound: NetworkDirection<V, S>,
{
    /// Create a new [`ConnectionStateEvent`].
    #[inline]
    #[must_use]
    pub fn new(entity: Entity, connection: Connection<V, S, Clientbound>) -> Self {
        Self { entity, connection: Mutex::new(Some(connection)) }
    }

    /// Take the [`Connection`] from the [`ConnectionStateEvent`].
    ///
    /// Can only be called once, otherwise it will always return `None`.
    #[inline]
    #[must_use]
    pub fn take(&self) -> Option<Connection<V, S, Clientbound>> { self.connection.lock().take() }
}

/// An [`Event`] that is sent when a packet is received.
#[derive(Event)]
pub struct PacketEvent<V: Version, S: State<V>> {
    /// The [`Entity`] the packet is associated with.
    pub entity: Entity,
    /// The packet.
    pub packet: Arc<S::ServerboundPacket>,
}

impl<V: Version, S: State<V>> PacketEvent<V, S> {
    /// Create a new [`PacketEvent`].
    #[inline]
    #[must_use]
    pub const fn new(entity: Entity, packet: Arc<S::ServerboundPacket>) -> Self {
        Self { entity, packet }
    }
}
