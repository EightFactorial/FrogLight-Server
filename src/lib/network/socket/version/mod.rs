use std::{future::Future, sync::Arc, time::Duration};

use async_channel::Sender;
use async_std::net::TcpListener;
use froglight::{network::connection::NetworkDirection, prelude::*};
use parking_lot::RwLock;

use super::ConnectionRequest;

mod v1_21_0;

/// A trait that defines the behavior of a socket.
pub trait SocketTrait: Version
where
    Clientbound: NetworkDirection<Self, Login>,
    Login: State<Self>,
{
    /// The timeout for the listener.
    const TIMEOUT: Duration = Duration::from_secs(5);
    /// The maximum number of status packets that can be sent.
    const MAX_PACKETS: usize = 3;

    /// The default status of the server.
    fn status() -> ServerStatus;

    /// An async function that listens for incoming connections.
    fn listen(
        listener: TcpListener,
        status: Arc<RwLock<ServerStatus>>,
        channel: Sender<ConnectionRequest<Self>>,
    ) -> impl Future<Output = ()> + Send + Sync;
}
