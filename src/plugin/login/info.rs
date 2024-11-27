use std::net::SocketAddr;

use bevy::prelude::*;
use froglight::prelude::ConnectionIntent;

/// Information about a connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct ConnectionInformation {
    /// The protocol version of the client.
    pub protocol: i32,
    /// The intent of the connection.
    pub intent: ConnectionIntent,
    /// The socket address of the client.
    pub socket: SocketAddr,
}
