//! TODO

use bevy::{prelude::*, utils::HashMap};
use derive_more::derive::Debug;
use froglight::{
    network::{
        connection::{AccountInformation, NetworkDirection},
        versions::v1_21_0::V1_21_0,
    },
    prelude::{State, *},
};
use parking_lot::Mutex;

use super::{listen::ConnectionRequest, AcceptedConnectionEvent, ConnectionFilterPlugin};

mod channel;
pub use channel::{channel, AsyncLoginChannel, TaskLoginChannel};

mod event;
pub use event::ConnectionLoginEvent;

mod info;
pub use info::ConnectionInformation;

mod task;
pub use task::LoginTask;

/// A plugin that manages logins to the server.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoginPlugin;

impl Plugin for LoginPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            Self::spawn_logins
                .after(ConnectionFilterPlugin::filter_requests)
                .run_if(on_event::<AcceptedConnectionEvent>),
        );

        app.add_event::<ConnectionLoginEvent<V1_21_0>>();
        app.add_systems(
            PostUpdate,
            Self::poll_logins::<V1_21_0>.run_if(any_with_component::<LoginTask<V1_21_0>>),
        );
    }
}

/// The target for this module.
static TARGET: &str = "CON";

impl LoginPlugin {
    /// Spawn [`LoginTask`]s for
    /// [`AcceptedConnections`](AcceptedConnectionEvent).
    pub fn spawn_logins(mut events: EventReader<AcceptedConnectionEvent>, mut command: Commands) {
        for AcceptedConnectionEvent {
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
            let task = LoginTask::spawn(connection);

            // Spawn the entity
            let entity = command.spawn((profile, information, task));
            debug!(target: TARGET, "Assigning {username} to Entity {}", entity.id());
        }
    }

    /// Poll [`LoginTask`]s for completion.
    ///
    /// Sends a [`ConnectionLoginEvent`] when a task completes.
    ///
    /// # TODO
    /// Prevent clients logging in by sending a `EnterConfiguration` packet.
    pub fn poll_logins<V: Version>(
        mut query: Query<(Entity, &GameProfile, &mut LoginTask<V>)>,
        mut events: EventWriter<ConnectionLoginEvent<V>>,
        mut commands: Commands,
    ) where
        Clientbound: NetworkDirection<V, Login> + NetworkDirection<V, Configuration>,
        Login: State<V>,
        Configuration: State<V>,
    {
        for (entity, profile, mut task) in &mut query {
            match task.poll() {
                Some(Ok(connection)) => {
                    info!(target: TARGET, "Successfully logged in {}", profile.name);
                    commands.entity(entity).remove::<LoginTask<V>>();

                    let connection = Mutex::new(Some(connection));
                    events.send(ConnectionLoginEvent { entity, connection });
                }
                Some(Err(error)) => {
                    error!(target: TARGET, "Failed to log in {}: {error}", profile.name);
                }
                None => {}
            }
        }
    }
}
