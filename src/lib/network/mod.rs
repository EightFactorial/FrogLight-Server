//! TODO

use std::{marker::PhantomData, net::SocketAddr};

use bevy::app::{Plugin, PluginGroup, PluginGroupBuilder};
use froglight::prelude::Version;

mod common;
pub use common::*;

mod config;
pub use config::*;

mod login;
pub use login::*;

mod play;
pub use play::*;

mod socket;
pub use socket::*;

/// A [`PluginGroup`] for managing network connections.
///
/// Contains:
/// - [`SocketPlugin`]
/// - [`LoginPlugin`]
/// - [`ConfigPlugin`]
/// - [`PlayPlugin`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetworkPlugins<V: Version> {
    socket: SocketAddr,
    _phantom: PhantomData<V>,
}

impl<V: Version> NetworkPlugins<V> {
    /// Create a new [`NetworkPlugins`] group with the given [`SocketAddr`].
    #[must_use]
    pub const fn from_socket(socket: SocketAddr) -> Self { Self { socket, _phantom: PhantomData } }
}

impl<V: Version> PluginGroup for NetworkPlugins<V>
where
    SocketPlugin<V>: Plugin,
{
    fn build(self) -> PluginGroupBuilder {
        let builder = PluginGroupBuilder::start::<Self>();
        builder
            .add(SocketPlugin::<V>::from_socket(self.socket))
            .add(LoginPlugin::<V>::default())
            .add(ConfigPlugin::<V>::default())
            .add(PlayPlugin::<V>::default())
    }
}
