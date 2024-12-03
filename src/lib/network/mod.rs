//! TODO

use std::{marker::PhantomData, net::SocketAddr};

use bevy::{app::PluginGroupBuilder, prelude::*};
use froglight::prelude::Version;

pub mod common;

pub mod socket;
pub use socket::SocketPlugin;

/// A [`PluginGroup`] that adds network-related plugins to the app.
///
/// When given a [`SocketAddr`] it will listen on it for incoming connections.
#[derive(Debug, Default)]
pub struct NetworkPlugins<V: Version> {
    /// The socket to listen on.
    pub socket: Option<SocketAddr>,
    _phantom: PhantomData<V>,
}

impl<V: Version> NetworkPlugins<V> {
    /// Create a new [`NetworkPlugins`] [`PluginGroup`].
    #[must_use]
    pub const fn new() -> Self { Self::from_option(None) }

    /// Create a new [`NetworkPlugins`] [`PluginGroup`]
    /// that listens on the given socket.
    #[must_use]
    pub const fn from_socket(socket: SocketAddr) -> Self { Self::from_option(Some(socket)) }

    /// Set the socket to listen on.
    #[must_use]
    pub const fn with_socket(mut self, socket: SocketAddr) -> Self {
        self.socket = Some(socket);
        self
    }

    /// Create a new [`NetworkPlugins`] [`PluginGroup`]
    /// that listens on the given socket, if it is [`Some`].
    #[must_use]
    pub const fn from_option(socket: Option<SocketAddr>) -> Self {
        Self { socket, _phantom: PhantomData }
    }
}
impl<V: Version> From<SocketAddr> for NetworkPlugins<V> {
    fn from(socket: SocketAddr) -> Self { Self::from_socket(socket) }
}

impl<V: Version> PluginGroup for NetworkPlugins<V>
where
    SocketPlugin<V>: Plugin,
{
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();

        // If the `SocketAddr` is set, add the `SocketPlugin`.
        if let Some(socket) = self.socket {
            builder = builder.add(SocketPlugin::<V>::from_socket(socket));
        }

        builder
    }
}
