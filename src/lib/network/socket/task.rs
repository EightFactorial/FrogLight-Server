use std::{net::SocketAddr, sync::Arc};

use async_channel::{Receiver, TryRecvError};
use bevy::{
    ecs::system::SystemState,
    prelude::*,
    tasks::{block_on, poll_once, Task},
};
use compact_str::{CompactString, ToCompactString};
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};
use parking_lot::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::{ConnectionRequestEvent, SocketFilter};
use crate::network::{socket::TARGET, FilterResult};

/// A component that listens for incoming connections.
#[derive(Component)]
pub struct ConnectionListener<V: Version>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    pub(super) receiver: Receiver<ConnectionRequest<V>>,
    pub(super) status: Arc<RwLock<ServerStatus>>,
    pub(super) task: Task<()>,
}

impl<V: Version> ConnectionListener<V>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    /// Receive a connection request.
    #[must_use]
    pub fn recv(&self) -> Option<ConnectionRequest<V>> { self.try_recv().ok() }
    /// Try to receive a connection request.
    ///
    /// # Errors
    /// Returns an error if the channel has been closed.
    pub fn try_recv(&self) -> Result<ConnectionRequest<V>, TryRecvError> {
        self.receiver.try_recv()
    }

    /// A shared reference to the [`ServerStatus`].
    pub fn status(&self) -> RwLockReadGuard<ServerStatus> { self.status.read() }

    /// A mutable reference to the [`ServerStatus`].
    pub fn status_mut(&self) -> RwLockWriteGuard<ServerStatus> { self.status.write() }

    /// Poll the [`ConnectionListener`] task.
    ///
    /// # Note
    /// This will panic if the task returns `Some` and is then polled again.
    pub fn poll(&mut self) -> Option<()> { block_on(poll_once(&mut self.task)) }
}

impl<V: Version> ConnectionListener<V>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    const DEFAULT_REASON: &'static str = "Denied by server";

    pub(super) fn poll_tasks(world: &mut World, mut cache: Local<Vec<ConnectionRequestEvent<V>>>) {
        let mut state =
            SystemState::<(Query<(Entity, &mut ConnectionListener<V>)>, Commands)>::new(world);
        let (mut query, mut commands) = state.get_mut(world);

        for (entity, mut listener) in &mut query {
            // Receive connection requests.
            while let Some(request) = listener.recv() {
                cache.push(ConnectionRequestEvent { entity, request });
            }

            // Poll the listener.
            if listener.poll().is_some() {
                warn!(target: TARGET, "{:?} listener exited, despawning {entity}", V::default());
                commands.entity(entity).despawn_recursive();
            }
        }

        state.apply(world);

        for event in cache.drain(..) {
            match world.resource::<SocketFilter<V>>().filter(&event.request, world) {
                FilterResult::Allow => {
                    info!(target: TARGET, "Accepted connection from {}", event.request.socket);
                    world.send_event(event);
                }
                FilterResult::Deny(reason) => {
                    let reason = reason.unwrap_or(Self::DEFAULT_REASON.to_compact_string());
                    warn!(target: TARGET, "Refused connection from {}: {reason}", event.request.socket);
                }
            }
        }
    }
}

/// A request to connect to the server.
pub struct ConnectionRequest<V: Version>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    /// The username of the client.
    pub username: CompactString,
    /// The UUID of the client.
    pub uuid: Uuid,
    /// Server address the client is connecting to.
    pub server: CompactString,
    /// The intent of the connection.
    pub intent: ConnectionIntent,
    /// The socket address of the client.
    pub socket: SocketAddr,
    /// The connection to the client.
    pub connection: Mutex<Option<Connection<V, Login, Clientbound>>>,
}

impl<V: Version> ConnectionRequest<V>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    /// Take the [`Connection`] from the request.
    #[must_use]
    pub fn take(&self) -> Option<Connection<V, Login, Clientbound>> {
        self.connection.lock().take()
    }
}
