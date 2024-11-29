use std::net::SocketAddr;

use bevy::prelude::Component;
use compact_str::CompactString;
use froglight::{network::connection::NetworkDirection, prelude::*};

use crate::network::ConnectionRequest;

/// Information about a [`Connection`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub struct ConnectionInfo {
    /// The address the client is connecting to.
    pub server: CompactString,
    /// The intent of the connection.
    pub intent: ConnectionIntent,
    /// The socket address of the client.
    pub socket: SocketAddr,
}

impl ConnectionInfo {
    /// Create a new [`ConnectionInfo`] from the given [`ConnectionRequest`].
    #[must_use]
    pub fn from_request<V: Version>(request: &ConnectionRequest<V>) -> Self
    where
        Clientbound: NetworkDirection<V, Login>,
        Login: State<V>,
    {
        Self::from(request)
    }
}

impl<V: Version> From<&ConnectionRequest<V>> for ConnectionInfo
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    fn from(value: &ConnectionRequest<V>) -> Self {
        Self { server: value.server.clone(), intent: value.intent, socket: value.socket }
    }
}
