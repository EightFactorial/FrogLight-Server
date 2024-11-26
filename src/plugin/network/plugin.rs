use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::app::{App, Plugin};

/// A plugin that sets up the network for the server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetworkPlugin {
    /// The address the server will bind to.
    pub socket: SocketAddr,
}

impl NetworkPlugin {
    /// The default socket address for the server.
    pub const LOCALHOST: SocketAddr =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25565);
}
impl Default for NetworkPlugin {
    fn default() -> Self { Self { socket: Self::LOCALHOST } }
}

impl Plugin for NetworkPlugin {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) { self.bind(app); }
}
