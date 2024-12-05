//! TODO

use std::{
    marker::PhantomData,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

mod event;
pub use event::ConnectionRequestEvent;

mod filter;
pub use filter::SocketFilter;

mod version;
pub use version::SocketTrait;

mod task;
pub use task::{ConnectionRequest, ListenTask};

/// A [`Plugin`] that listens on a socket for incoming connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SocketPlugin<V: Version> {
    socket: SocketAddr,
    _phantom: PhantomData<V>,
}

impl<V: Version> SocketPlugin<V> {
    /// Create a new [`SocketPlugin`] that listens on the given socket.
    #[must_use]
    pub const fn new(socket: SocketAddr) -> Self { Self { socket, _phantom: PhantomData } }

    /// Create a new [`SocketPlugin`] that listens on the given socket.
    #[inline]
    #[must_use]
    pub const fn from_socket(socket: SocketAddr) -> Self { Self::new(socket) }

    /// A [`SocketAddr`] that listens on [`Ipv4Addr::LOCALHOST`]
    /// and port `25565`.
    pub const LOCALHOST: SocketAddr = SocketAddr::new(Self::LOCALHOST_ADDR, 25565);
    const LOCALHOST_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

    /// Create a new [`SocketPlugin`] that listens on [`Self::LOCALHOST`].
    #[inline]
    #[must_use]
    pub const fn localhost() -> Self { Self::from_socket(Self::LOCALHOST) }

    /// A [`SocketAddr`] that listens on [`Ipv4Addr::UNSPECIFIED`]
    /// and port `25565`.
    pub const PUBLIC: SocketAddr = SocketAddr::new(Self::PUBLIC_ADDR, 25565);
    const PUBLIC_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::UNSPECIFIED);

    /// Create a new [`SocketPlugin`] that listens on [`Self::PUBLIC`].
    #[inline]
    #[must_use]
    pub const fn public() -> Self { Self::from_socket(Self::PUBLIC) }
}
impl<V: Version> From<SocketAddr> for SocketPlugin<V> {
    fn from(socket: SocketAddr) -> Self { Self::from_socket(socket) }
}

impl<V: Version + SocketTrait> Plugin for SocketPlugin<V>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    fn build(&self, app: &mut App) {
        // Add events and initialize resources
        app.add_event::<ConnectionRequestEvent<V>>();
        app.init_resource::<SocketFilter<V>>();

        // Add systems
        app.add_systems(
            PreUpdate,
            ListenTask::<V>::receive_requests
                .run_if(any_with_component::<ListenTask<V>>)
                .ambiguous_with_all(),
        );
        app.add_systems(
            PostUpdate,
            ListenTask::<V>::poll_tasks
                .run_if(any_with_component::<ListenTask<V>>)
                .ambiguous_with_all(),
        );
    }

    fn finish(&self, app: &mut App) {
        match ListenTask::<V>::new(self.socket, None) {
            Ok(task) => {
                app.world_mut().spawn(task);
            }
            Err(err) => {
                error!("Failed to spawn {:?} listener: {err}", V::default());
            }
        }
    }
}
