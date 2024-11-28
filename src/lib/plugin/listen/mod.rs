//! TODO

use std::num::NonZero;

use bevy::prelude::*;

mod bind;
pub use bind::{ConnectionListener, ConnectionRequest};

mod plugin;
pub use plugin::ListenerPlugin;

mod status;
pub use status::ServerStatusArc;

/// The target for this module.
static TARGET: &str = "NET";

impl ListenerPlugin {
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
