//! TODO
#![allow(clippy::module_inception)]

use bevy::{prelude::*, tasks::IoTaskPool};
use froglight::{
    network::versions::v1_21_0::{play::DisconnectPacket, V1_21_0},
    prelude::*,
};
use simdnbt::owned::NbtTag;

mod filter;
pub use filter::{ConnectionFilter, FilterEntry, FilterMode};

mod list;
pub use list::{BoxedFilter, ConnectionFilterList, FilterFn, FilterResult};

mod ratelimit;
pub use ratelimit::RateLimitFilter;

use super::listen::{ConnectionListener, ConnectionRequest};

/// The target for this module.
static TARGET: &str = "FILTR";

/// A plugin that manages connections to the server.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionFilterPlugin;

impl Plugin for ConnectionFilterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConnectionFilterList>();
        app.init_resource::<ConnectionFilter>();
        app.init_resource::<RateLimitFilter>();

        app.add_event::<AcceptedConnectionEvent>();

        app.add_systems(PreUpdate, RateLimitFilter::tick_ratelimit);
        app.add_systems(PostUpdate, ConnectionFilterPlugin::filter_requests);
    }
}

impl ConnectionFilterPlugin {
    /// The default reason for denying a connection.
    const DEFAULT_REASON: &str = "Connection denied";

    /// Filters incoming connection requests.
    ///
    /// Emits an [`AcceptedConnection`] event for each accepted request.
    pub fn filter_requests(world: &mut World, mut cache: Local<Vec<ConnectionRequest>>) {
        let Some(listener) = world.get_resource::<ConnectionListener>() else {
            error_once!(target: TARGET, "No connection listener found");
            return;
        };
        let Some(filter) = world.get_resource::<ConnectionFilterList>() else {
            error_once!(target: TARGET, "No connection filters found");
            return;
        };

        // Accept or deny incoming connection requests
        while let Some(request) = listener.recv() {
            match filter.passes(&request, world) {
                // Accept the connection
                FilterResult::Allow => {
                    info!(target: TARGET, "Accepted connection from {}", request.username);
                    cache.push(request);
                }
                // Drop the connection
                FilterResult::Deny(reason) => {
                    let reason = reason.unwrap_or(Self::DEFAULT_REASON.to_string());
                    warn!(target: TARGET, "Denied connection from {}: {reason}", request.username);

                    if let Some(connection) = std::mem::take(&mut *request.connection.lock()) {
                        IoTaskPool::get().spawn(send_disconnect(connection, reason)).detach();
                    }
                }
            }
        }

        // Send `AcceptedConnection` events for each accepted request
        for request in cache.drain(..) {
            world.send_event(AcceptedConnectionEvent { request });
        }
    }
}

/// An event that is fired when a [`ConnectionRequest`] is accepted.
#[derive(Event)]
pub struct AcceptedConnectionEvent {
    /// The connection request that was accepted.
    pub request: ConnectionRequest,
}

/// Sends a disconnect packet to the client.
async fn send_disconnect(mut connection: Connection<V1_21_0, Login, Clientbound>, reason: String) {
    if let Err(err) =
        connection.send(DisconnectPacket { reason: NbtTag::String(reason.into()) }).await
    {
        error!(target: TARGET, "Failed to send disconnect: {err}");
    }
}
