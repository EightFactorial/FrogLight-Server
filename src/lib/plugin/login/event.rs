use std::sync::Arc;

use bevy::prelude::{Entity, Event};
use froglight::{network::connection::NetworkDirection, prelude::*};
use parking_lot::Mutex;

/// An [`Event`] that is fired when a [`Connection`] has logged in.
#[derive(Event)]
pub struct ConnectionLoginEvent<V: Version>
where
    Clientbound: NetworkDirection<V, Configuration>,
    Configuration: State<V>,
{
    /// The [`Entity`] assigned to the [`Connection`].
    pub entity: Entity,
    /// The [`Connection`] that has logged in.
    pub connection: Mutex<Option<Connection<V, Configuration, Clientbound>>>,
}

/// An [`Event`] that is fired when a [`Login`] packet is received.
#[derive(Event)]
pub struct LoginPacketEvent<V: Version>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    /// The [`Entity`] assigned to the [`Connection`].
    pub entity: Entity,
    /// The [`Login`] packet that was received.
    pub packet: Arc<<Login as State<V>>::ServerboundPacket>,
}
