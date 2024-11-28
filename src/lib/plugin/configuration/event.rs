use std::sync::Arc;

use bevy::prelude::{Entity, Event};
use froglight::{network::connection::NetworkDirection, prelude::*};
use parking_lot::Mutex;

/// An [`Event`] that is fired when a [`Connection`] has been configured.
#[derive(Event)]
pub struct ConnectionConfigEvent<V: Version>
where
    Clientbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    /// The [`Entity`] assigned to the [`Connection`].
    pub entity: Entity,
    /// The [`Connection`] that has logged in.
    pub connection: Mutex<Option<Connection<V, Play, Clientbound>>>,
}

/// An [`Event`] that is fired when a [`Configuration`] packet is received.
#[derive(Event)]
pub struct ConfigPacketEvent<V: Version>
where
    Clientbound: NetworkDirection<V, Configuration>,
    Configuration: State<V>,
{
    /// The [`Entity`] assigned to the [`Connection`].
    pub entity: Entity,
    /// The [`Config`] packet that was received.
    pub packet: Arc<<Configuration as State<V>>::ServerboundPacket>,
}
