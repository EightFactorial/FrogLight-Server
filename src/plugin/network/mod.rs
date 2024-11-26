//! TODO

use std::num::NonZero;

use bevy::prelude::*;

mod listen;
pub use listen::{ConnectionListener, ConnectionRequest};

mod plugin;
pub use plugin::NetworkPlugin;

mod status;
pub use status::ServerStatusArc;

impl NetworkPlugin {
    fn bind(&self, app: &mut App) {
        // Get or create the `ServerStatusArc`
        let status = app.world_mut().get_resource_or_init::<ServerStatusArc>();

        // Create a new `ConnectionListener` and insert it.
        match ConnectionListener::new(self.socket, status.clone()) {
            Ok(listener) => {
                app.insert_resource(listener);
            }
            Err(err) => {
                error!("NET : {err}");
                app.world_mut().send_event(AppExit::Error(NonZero::new(1).unwrap()));
            }
        }
    }
}
