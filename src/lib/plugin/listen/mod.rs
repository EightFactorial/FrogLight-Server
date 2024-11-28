//! TODO

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    num::NonZero,
};

use bevy::{
    app::{App, Plugin},
    prelude::*,
};

mod bind;
pub use bind::{ConnectionListener, ConnectionRequest};

mod status;
pub use status::ServerStatusArc;

/// A plugin that sets up listening for incoming connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ListenerPlugin {
    /// The address the server will bind to.
    pub socket: SocketAddr,
}

impl Default for ListenerPlugin {
    fn default() -> Self { Self { socket: Self::LOCALHOST } }
}
impl Plugin for ListenerPlugin {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) { self.bind(app); }
}

/// The target for this module.
static TARGET: &str = "NETWK";

impl ListenerPlugin {
    /// The default socket address for the server.
    pub const LOCALHOST: SocketAddr =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25565);

    fn bind(&self, app: &mut App) {
        // Get or create the `ServerStatusArc`
        let status = app.world_mut().get_resource_or_init::<ServerStatusArc>();

        // Create a new `ConnectionListener` and bind to the socket
        match ConnectionListener::new(self.socket, status.clone()) {
            // Insert the listener resource
            Ok(listener) => {
                app.insert_resource(listener);
            }
            // Log the error and exit
            Err(err) => {
                error!(target: TARGET, "{err}");
                app.world_mut().send_event(AppExit::Error(NonZero::new(1).unwrap()));
            }
        }
    }
}
