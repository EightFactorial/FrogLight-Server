use std::{net::SocketAddr, sync::Arc};

use async_channel::Receiver;
use async_std::net::TcpListener;
use bevy::{
    ecs::system::SystemState,
    prelude::*,
    tasks::{block_on, poll_once, IoTaskPool, Task},
};
use compact_str::CompactString;
use froglight::{
    network::connection::{ConnectionInformation, NetworkDirection},
    prelude::{State, *},
};
use parking_lot::{Mutex, RwLock};

use super::{ConnectionRequestEvent, SocketFilter, SocketTrait};
use crate::network::common::FilterResult;

/// A task that listens for incoming connections.
#[derive(Component)]
pub struct ListenTask<V: Version>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    recv: Receiver<ConnectionRequest<V>>,
    status: Arc<RwLock<ServerStatus>>,
    task: Task<()>,
}

impl<V: Version> ListenTask<V>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    /// Create a new [`ListenTask`] that listens on the given socket.
    ///
    /// # Errors
    /// Returns an error if the [`TcpListener`] fails to bind to the socket.
    pub fn new(socket: SocketAddr, status: Option<ServerStatus>) -> Result<Self, std::io::Error>
    where
        V: SocketTrait,
    {
        let listener = block_on(TcpListener::bind(socket))?;
        let status = Arc::new(RwLock::new(status.unwrap_or_else(V::status)));

        let (send, recv) = async_channel::unbounded();
        let task = IoTaskPool::get().spawn(V::listen(listener, status.clone(), send));

        Ok(Self { recv, status, task })
    }

    /// Try to receive any incoming connection requests.
    #[inline]
    #[must_use]
    pub fn recv(&self) -> Option<ConnectionRequest<V>> { self.try_recv().ok() }

    /// Try to receive any incoming connection requests.
    ///
    /// # Errors
    /// Returns an error if the channel is empty or closed.
    pub fn try_recv(&self) -> Result<ConnectionRequest<V>, async_channel::TryRecvError> {
        self.recv.try_recv()
    }

    /// Get the status of the server.
    ///
    /// The [`ServerStatus`] represents what clients who
    /// request the server's status will see.
    #[must_use]
    pub fn status(&self) -> &RwLock<ServerStatus> { &self.status }

    /// Poll the listener task once.
    ///
    /// # Warning
    /// If this returns `Some` and is polled again it will panic.
    ///
    /// The task must be dropped before it can be polled again.
    #[must_use]
    pub fn poll(&mut self) -> Option<()> { block_on(poll_once(&mut self.task)) }

    const DEFAULT_REASON: &'static str = "Denied by server";

    /// A system that receives, filters, and sends connection request events.
    pub fn receive_requests(world: &mut World, mut cache: Local<Vec<ConnectionRequestEvent<V>>>) {
        // Query for all incoming connection requests and filter them.
        {
            let mut state =
                SystemState::<(Query<(Entity, &ListenTask<V>)>, Res<SocketFilter<V>>)>::new(world);
            let (query, filters) = state.get(world);

            for (entity, task) in &query {
                while let Ok(request) = task.try_recv() {
                    match filters.check(&request, world) {
                        FilterResult::Allow => {
                            info!(
                                "Incoming connection from {} ({})",
                                request.username, request.information.socket
                            );
                            cache.push(ConnectionRequestEvent { listener: entity, request });
                        }
                        FilterResult::Deny(reason) => {
                            let reason = reason.as_deref().unwrap_or(Self::DEFAULT_REASON);
                            warn!(
                                "Denied connection request from {} ({}): {reason}",
                                request.username, request.information.socket
                            );
                        }
                    }
                }
            }
            state.apply(world);
        }

        // Create events for all passing connection requests.
        world.send_event_batch(cache.drain(..));
    }

    /// A system that polls all listener tasks and
    /// despawns them if they are done.
    pub fn poll_tasks(mut query: Query<(Entity, &mut ListenTask<V>)>, mut commands: Commands) {
        for (entity, mut task) in &mut query {
            if let Some(()) = task.poll() {
                commands.entity(entity).despawn();
            }
        }
    }
}

/// A request to connect to a server.
pub struct ConnectionRequest<V: Version>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    /// The username of the client.
    pub username: CompactString,
    /// The UUID of the client.
    pub uuid: Uuid,
    /// Information about the connection.
    pub information: ConnectionInformation,
    /// The connection to the server.
    pub connection: Mutex<Option<Connection<V, Login, Clientbound>>>,
}
