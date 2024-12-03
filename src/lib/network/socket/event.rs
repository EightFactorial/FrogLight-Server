use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use super::task::ConnectionRequest;

/// An event that is triggered when a [`ConnectionRequest`] is received.
#[derive(Event)]
pub struct ConnectionRequestEvent<V: Version>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    /// The listener entity that received the request.
    pub listener: Entity,
    /// The connection request.
    pub request: ConnectionRequest<V>,
}
