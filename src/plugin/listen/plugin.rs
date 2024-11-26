use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::app::{App, Plugin};

/// A plugin that sets up listening for incoming connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ListenerPlugin {
    /// The address the server will bind to.
    pub socket: SocketAddr,
}

impl ListenerPlugin {
    /// The default socket address for the server.
    pub const LOCALHOST: SocketAddr =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25565);
}
impl Default for ListenerPlugin {
    fn default() -> Self { Self { socket: Self::LOCALHOST } }
}

impl Plugin for ListenerPlugin {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) { self.bind(app); }
}
