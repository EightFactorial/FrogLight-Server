use bevy::prelude::{Entity, Event};
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use super::ConnectionRequest;

/// An [`Event`] that is sent when a [`ConnectionRequest`] is received.
#[derive(Event)]
pub struct ConnectionRequestEvent<V: Version>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    /// The listener that received the request.
    pub entity: Entity,
    /// The connection request.
    pub request: ConnectionRequest<V>,
}
