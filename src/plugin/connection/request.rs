use bevy::prelude::*;

use super::{ConnectionFilterList, FilterResult};
use crate::plugin::{connection::TARGET, listen::ConnectionListener};

/// The default reason for denying a connection.
const DEFAULT_REASON: &str = "Connection denied";

pub(super) fn poll_connection_requests(world: &World) {
    let Some(listener) = world.get_resource::<ConnectionListener>() else {
        error_once!(target: TARGET, "No connection listener found");
        return;
    };
    let Some(filter) = world.get_resource::<ConnectionFilterList>() else {
        error_once!(target: TARGET, "No connection filters found");
        return;
    };

    while let Some(request) = listener.recv() {
        match filter.passes(&request, world) {
            // Accept the connection
            FilterResult::Allow => {
                info!(target: TARGET, "Accepted connection from {}", request.username);
            }
            // Drop the connection
            FilterResult::Deny(reason) => {
                let reason = reason.as_deref().unwrap_or(DEFAULT_REASON);
                warn!(target: TARGET, "Denied connection from {}: {reason}", request.username);
            }
        }
    }
}
