use std::{future::Future, net::SocketAddr, sync::Arc, time::Duration};

use async_channel::Sender;
use async_std::net::TcpListener;
use bevy::{
    log::info,
    tasks::{block_on, IoTaskPool},
};
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};
use parking_lot::RwLock;

use super::{ConnectionListener, ConnectionRequest};
use crate::network::socket::TARGET;

mod v1_21_0;

/// A trait for connection listeners.
pub trait ListenerTrait: Version
where
    Clientbound: NetworkDirection<Self, Login>,
    Login: State<Self>,
{
    /// The timeout for the listener.
    const TIMEOUT: Duration = Duration::from_secs(5);
    /// The maximum number of status packets that can be sent.
    const MAX_PACKETS: usize = 3;

    /// The default server status when none is provided.
    fn default_status() -> ServerStatus;

    /// Create a new [`ConnectionListener`] that listens on the given socket.
    ///
    /// # Errors
    /// Returns an error if the listener cannot bind to the socket.
    #[inline]
    fn new(socket: SocketAddr) -> Result<ConnectionListener<Self>, std::io::Error> {
        Self::new_from(socket, Arc::new(RwLock::new(Self::default_status())))
    }

    /// Create a new [`ConnectionListener`] that listens on the given socket.
    ///
    /// # Errors
    /// Returns an error if the listener cannot bind to the socket.
    fn new_from(
        socket: SocketAddr,
        status: Arc<RwLock<ServerStatus>>,
    ) -> Result<ConnectionListener<Self>, std::io::Error> {
        let (sender, receiver) = async_channel::unbounded();
        let task_status = status.clone();

        let listener = block_on(TcpListener::bind(socket))?;
        let task = IoTaskPool::get().spawn(Self::listen(listener, task_status, sender));
        info!(target: TARGET, "Listening on {socket}");

        Ok(ConnectionListener { receiver, status, task })
    }

    /// An async function that listens for incoming connections.
    fn listen(
        listener: TcpListener,
        status: Arc<RwLock<ServerStatus>>,
        channel: Sender<ConnectionRequest<Self>>,
    ) -> impl Future<Output = ()> + Send + 'static;
}
