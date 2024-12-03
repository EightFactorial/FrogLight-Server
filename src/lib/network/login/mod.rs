//! TODO

use std::marker::PhantomData;

use bevy::prelude::*;
use compact_str::CompactString;
use froglight::{network::connection::NetworkDirection, prelude::*};

use super::socket::ConnectionRequestEvent;
use crate::dimension::{All, DimensionApp};

mod version;
pub use version::LoginTrait;

mod task;

mod types;
pub use types::*;

/// A [`Plugin`] that listens for incoming connections and authenticates them.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoginPlugin<V: Version> {
    /// The address of the authentication server.
    pub auth_server: Option<CompactString>,
    _phantom: PhantomData<V>,
}

impl<V: Version> LoginPlugin<V> {
    /// The default authentication server.
    pub const MOJANG_SERVER: CompactString =
        CompactString::const_new("https://sessionserver.mojang.com");

    /// Create a new [`LoginPlugin`] without authentication.
    #[must_use]
    pub const fn offline() -> Self { Self { auth_server: None, _phantom: PhantomData } }

    /// Create a new [`LoginPlugin`] using online authentication.
    #[must_use]
    pub const fn online() -> Self {
        Self { auth_server: Some(Self::MOJANG_SERVER), _phantom: PhantomData }
    }

    /// Create a new [`LoginPlugin`] using custom authentication.
    #[must_use]
    pub const fn from_address(server: CompactString) -> Self {
        Self { auth_server: Some(server), _phantom: PhantomData }
    }

    /// Create a new [`LoginPlugin`] optionally using custom authentication.
    #[must_use]
    pub const fn from_option(server: Option<CompactString>) -> Self {
        Self { auth_server: server, _phantom: PhantomData }
    }
}

impl<V: Version + LoginTrait> Plugin for LoginPlugin<V>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    fn build(&self, app: &mut App) {
        // Insert the `AuthenticationServer`
        let auth_server = AuthenticationServer::<V>::from(self.auth_server.clone());
        app.insert_dimension_resource(All, auth_server.clone());
        app.insert_resource(auth_server);

        // Add events and initialize resources
        app.add_event::<LoginStateEvent<V>>();
        app.add_event::<LoginPacketEvent<V>>();
        app.init_resource::<LoginFilter<V>>();

        // Initialize and add required components
        let mut required = LoginRequiredComponents::<V>::new_empty();
        required.add_required::<GameProfile>();
        app.insert_resource(required);

        // Add systems
        app.add_systems(
            PreUpdate,
            LoginTask::<V>::receive_packets.run_if(any_with_component::<LoginTask<V>>),
        );
        app.add_systems(
            Update,
            LoginTask::<V>::complete_logins.run_if(any_with_component::<LoginTask<V>>),
        );
        app.add_systems(
            PostUpdate,
            (
                LoginTask::<V>::receive_requests.run_if(on_event::<ConnectionRequestEvent<V>>),
                LoginTask::<V>::poll_tasks.run_if(any_with_component::<LoginTask<V>>),
            ),
        );
    }
}
