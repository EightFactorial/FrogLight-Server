use std::{
    marker::PhantomData,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use bevy::{prelude::*, utils::HashSet};

mod event;
pub use event::ConnectionRequestEvent;

mod task;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};
pub use task::{ConnectionListener, ConnectionRequest};

mod filter;
pub use filter::SocketFilter;

mod listen;
pub use listen::ListenerTrait;

mod systemset;
pub use systemset::ListenSystemSet;

static TARGET: &str = "SOCK";

/// A plugin that listens for incoming connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct SocketPlugin<V: Version> {
    #[deref]
    socket: SocketAddr,
    _phantom: PhantomData<V>,
}

impl<V: Version> SocketPlugin<V> {
    /// A [`SocketAddr`] that listens on the local machine only.
    pub const LOCALHOST: SocketAddr = SocketAddr::new(Self::LOCALHOST_IP, Self::DEFAULT_PORT);
    const LOCALHOST_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    /// A [`SocketAddr`] that listens on all interfaces.
    pub const PUBLIC: SocketAddr = SocketAddr::new(Self::PUBLIC_IP, Self::DEFAULT_PORT);
    const PUBLIC_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

    /// The default port for the server.
    pub const DEFAULT_PORT: u16 = 25565;

    /// Create a new [`SocketPlugin`] that listens on the local machine only.
    #[must_use]
    pub const fn localhost() -> Self { Self::from_socket(Self::LOCALHOST) }

    /// Create a new [`SocketPlugin`] that listens on all interfaces.
    #[must_use]
    pub const fn public() -> Self { Self::from_socket(Self::PUBLIC) }

    /// Create a new [`SocketPlugin`] with the given [`SocketAddr`].
    #[must_use]
    pub const fn from_socket(socket: SocketAddr) -> Self { Self { socket, _phantom: PhantomData } }
}

impl<V: Version + ListenerTrait> Plugin for SocketPlugin<V>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    fn build(&self, app: &mut App) {
        // Only configure the `ListenSystemSet` if it doesn't already exist
        if app
            .get_schedule(PostUpdate)
            .is_some_and(|sched| !sched.graph().contains_set(ListenSystemSet))
        {
            app.configure_sets(PostUpdate, ListenSystemSet);
        }

        app.add_event::<ConnectionRequestEvent<V>>();
        app.init_resource::<SocketFilter<V>>();
        app.init_resource::<NetworkListenerBinds>();

        app.add_systems(
            PostUpdate,
            ConnectionListener::<V>::poll_listeners
                .run_if(any_with_component::<ConnectionListener<V>>)
                .in_set(ListenSystemSet),
        );
    }

    /// Finish adding this plugin to the [`App`], once all plugins registered
    /// are ready. This can be useful for plugins that depends on another
    /// plugin asynchronous setup, like the renderer.
    ///
    /// # Note
    /// Only **one** version listener should be added.
    ///
    /// The [`SocketPlugin`] only has one [`SocketAddr`], so if
    /// multiple versions are added any subsequent versions will fail.
    fn finish(&self, app: &mut App) {
        let binds = app.world().resource::<NetworkListenerBinds>();
        if binds.contains(&self.socket) {
            warn!(target: TARGET, "Skipping {:?} listener, socket is already bound", V::default());
            return;
        }

        match <V as ListenerTrait>::new(**self) {
            Ok(listener) => {
                let entity = app.world_mut().spawn(listener);
                debug!(target: TARGET, "Spawned {:?} listener on Entity {}", V::default(), entity.id());
                app.world_mut().resource_mut::<NetworkListenerBinds>().insert(self.socket);
            }
            Err(err) => {
                error!(target: TARGET, "Failed to create {:?} listener: {err}", V::default());
                app.world_mut().send_event(AppExit::error());
            }
        }
    }
}

/// A [`Resource`] that stores which [`SocketAddr`]s that are bound.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct NetworkListenerBinds {
    binds: HashSet<SocketAddr>,
}

impl<V: Version> From<SocketAddr> for SocketPlugin<V> {
    fn from(socket: SocketAddr) -> Self { Self { socket, _phantom: PhantomData } }
}
impl<V: Version> From<SocketPlugin<V>> for SocketAddr {
    fn from(plugin: SocketPlugin<V>) -> Self { plugin.socket }
}
