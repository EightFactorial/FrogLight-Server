//! TODO

use std::net::SocketAddr;

use bevy::{
    prelude::*,
    tasks::{IoTaskPool, Task},
    utils::HashMap,
};
use derive_more::derive::Debug;
use froglight::{
    network::{
        connection::AccountInformation,
        versions::v1_21_0::{
            configuration::{ConfigurationServerboundPackets, ReadyS2CPacket},
            login::{LoginServerboundPackets, LoginSuccessPacket},
            V1_21_0,
        },
    },
    prelude::*,
};

use super::{listen::ConnectionRequest, AcceptedConnection, ConnectionFilterPlugin};

/// A plugin that manages logins to the server.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoginPlugin;

impl Plugin for LoginPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            Self::perform_connection_login
                .after(ConnectionFilterPlugin::filter_requests)
                .run_if(on_event::<AcceptedConnection>),
        );
    }
}

/// The target for this module.
static TARGET: &str = "CON";

impl LoginPlugin {
    fn perform_connection_login(
        mut events: EventReader<AcceptedConnection>,
        mut command: Commands,
    ) {
        for AcceptedConnection {
            request: ConnectionRequest { username, protocol, intent, socket, connection, .. },
        } in events.read()
        {
            let Some(connection) = std::mem::take(&mut *connection.lock()) else {
                warn!(target: TARGET, "Connection was stolen before login");
                continue;
            };

            // Create a `ConnectionInformation`
            let information =
                ConnectionInformation { protocol: *protocol, intent: *intent, socket: *socket };

            // Create a `GameProfile`
            let uuid = AccountInformation::offline_uuid(username);
            let profile = GameProfile { uuid, name: username.clone(), properties: HashMap::new() };

            // Create a `LoginTask`
            let task = LoginTask::new(connection, profile.clone());

            // Spawn the entity
            let entity = command.spawn((profile, information, task));
            debug!(target: TARGET, "Assigning {username} to Entity {}", entity.id());
        }
    }
}

/// A task that logs in a client.
#[derive(Component)]
#[expect(dead_code)]
pub struct LoginTask(
    Task<Result<Connection<V1_21_0, Configuration, Clientbound>, ConnectionError>>,
);

impl LoginTask {
    /// Create a new [`LoginTask`] that logs in a client.
    #[must_use]
    pub fn new(connection: Connection<V1_21_0, Login, Clientbound>, profile: GameProfile) -> Self {
        Self(IoTaskPool::get().spawn(Self::login(connection, profile)))
    }

    async fn login(
        mut connection: Connection<V1_21_0, Login, Clientbound>,
        profile: GameProfile,
    ) -> Result<Connection<V1_21_0, Configuration, Clientbound>, ConnectionError> {
        let username = profile.name.clone();

        // Client error, something about a ZipError?
        // connection.send(LoginCompressionPacket { threshold: 10 }).await?;
        connection.send(LoginSuccessPacket { profile, strict_error_handling: false }).await?;

        loop {
            match connection.recv().await {
                Ok(LoginServerboundPackets::EnterConfiguration(..)) => break,
                Ok(packet) => debug!(target: TARGET, "Received packet: {packet:?}"),
                Err(err) => {
                    error!(target: TARGET, "Failed to receive packet: {err}");
                    return Err(err);
                }
            }
        }

        info!(target: TARGET, "Successfully logged in {username}");

        let mut connection = connection.configuration();
        connection.send(ReadyS2CPacket).await?;

        loop {
            match connection.recv().await {
                Ok(ConfigurationServerboundPackets::Ready(..)) => break,
                Ok(packet) => debug!(target: TARGET, "Received packet: {packet:?}"),
                Err(err) => {
                    error!(target: TARGET, "Failed to receive packet: {err}");
                    return Err(err);
                }
            }
        }

        info!(target: TARGET, "Successfully configured {username}");
        todo!("Login properly")

        // Ok(connection)
    }
}

/// Information about a connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct ConnectionInformation {
    /// The protocol version of the client.
    pub protocol: i32,
    /// The intent of the connection.
    pub intent: ConnectionIntent,
    /// The socket address of the client.
    pub socket: SocketAddr,
}
