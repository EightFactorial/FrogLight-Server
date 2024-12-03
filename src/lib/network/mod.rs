//! TODO

use std::{marker::PhantomData, net::SocketAddr};

use bevy::{app::PluginGroupBuilder, prelude::*};
use compact_str::CompactString;
use froglight::{network::versions::v1_21_0::V1_21_0, prelude::Version};

pub mod common;

pub mod config;
pub use config::ConfigPlugin;

pub mod login;
pub use login::LoginPlugin;

pub mod socket;
pub use socket::SocketPlugin;

/// A [`PluginGroup`] that adds network-related plugins to the app.
///
/// When given a [`SocketAddr`] it will listen on it for incoming connections.
#[derive(Debug)]
pub struct NetworkPlugins<V: Version> {
    /// The socket to listen on.
    pub socket: Option<SocketAddr>,
    /// The address of the authentication server.
    pub auth_server: Option<CompactString>,

    _phantom: PhantomData<V>,
}

impl<V: Version> NetworkPlugins<V> {
    /// Create a new [`NetworkPlugins`] that listens on `127.0.0.1`.
    #[must_use]
    pub const fn localhost() -> Self { Self::from_socket(SocketPlugin::<V1_21_0>::LOCALHOST) }

    /// Create a new [`NetworkPlugins`] that listens on `0.0.0.0`.
    #[must_use]
    pub const fn public() -> Self { Self::from_socket(SocketPlugin::<V1_21_0>::PUBLIC) }

    /// Create a new [`NetworkPlugins`] [`PluginGroup`]
    /// that listens on the given socket.
    #[must_use]
    pub const fn from_socket(socket: SocketAddr) -> Self { Self::from_option(Some(socket)) }

    /// Create a new [`NetworkPlugins`] [`PluginGroup`]
    /// that listens on the given socket, if it is [`Some`].
    #[must_use]
    pub const fn from_option(socket: Option<SocketAddr>) -> Self {
        Self { socket, auth_server: None, _phantom: PhantomData }
    }

    /// Set the socket to listen on.
    #[must_use]
    pub const fn with_socket(mut self, socket: SocketAddr) -> Self {
        self.socket = Some(socket);
        self
    }

    /// Set the socket to listen on, if it is [`None`].
    #[must_use]
    pub const fn or_with_socket(self, socket: SocketAddr) -> Self {
        if self.socket.is_none() {
            self.with_socket(socket)
        } else {
            self
        }
    }

    /// Remove the authentication server.
    #[must_use]
    pub fn with_offline(mut self) -> Self {
        self.auth_server = None;
        self
    }

    /// Use the default Mojang authentication server.
    #[must_use]
    pub fn with_online(self) -> Self { self.with_auth(LoginPlugin::<V1_21_0>::MOJANG_SERVER) }

    /// Set the address of the authentication server.
    #[must_use]
    pub fn with_auth(mut self, auth_server: CompactString) -> Self {
        self.auth_server = Some(auth_server);
        self
    }

    /// Set the address of the authentication server, if it is [`None`].
    #[must_use]
    pub fn or_with_auth(self, auth_server: CompactString) -> Self {
        if self.auth_server.is_none() {
            self.with_auth(auth_server)
        } else {
            self
        }
    }
}

impl<V: Version> Default for NetworkPlugins<V> {
    fn default() -> Self { Self::localhost().with_offline() }
}

impl<V: Version> PluginGroup for NetworkPlugins<V>
where
    SocketPlugin<V>: Plugin,
    LoginPlugin<V>: Plugin,
    ConfigPlugin<V>: Plugin,
{
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();

        // If the `SocketAddr` is set, add the `SocketPlugin`.
        if let Some(socket) = self.socket {
            builder = builder.add(SocketPlugin::<V>::from(socket));
        }

        // Add the `LoginPlugin` using the configured authentication server.
        builder = builder.add(LoginPlugin::<V>::from_option(self.auth_server));
        // Add the `ConfigPlugin`.
        builder = builder.add(ConfigPlugin::<V>::default());

        builder
    }
}
