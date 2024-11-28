//! TODO

use action::{SendCompression, SendProfile};
use bevy::{ecs::system::SystemState, prelude::*, utils::HashMap};
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

mod action;
pub use action::{LoginCompressionAction, LoginProfileAction};

mod channel;
pub use channel::{channel, AsyncLoginChannel, TaskLoginChannel};

mod checklist;
pub use checklist::{LoginAction, LoginChecklist};

mod event;
pub use event::{ConnectionLoginEvent, LoginPacketEvent};

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

        Self::add_version::<V1_21_0>(app);
    }
}

/// The target for this module.
static TARGET: &str = "CON";

impl LoginPlugin {
    /// Add systems, resources, and events for the given [`Version`].
    fn add_version<V: Version + SendProfile + SendCompression>(app: &mut App)
    where
        Clientbound: NetworkDirection<V, Login> + NetworkDirection<V, Configuration>,
        Login: State<V>,
        Configuration: State<V>,
    {
        app.add_event::<ConnectionLoginEvent<V>>();
        app.add_event::<LoginPacketEvent<V>>();

        app.init_resource::<LoginChecklist<V>>();
        app.init_resource::<LoginCompressionAction>();

        app.add_systems(
            Update,
            (
                // LoginCompressionAction::send_login_compression::<V>,
                LoginProfileAction::send_login_profile::<V>,
            )
                .chain()
                .run_if(any_with_component::<LoginTask<V>>),
        );

        app.add_systems(
            PreUpdate,
            Self::login_packets::<V>.run_if(any_with_component::<LoginTask<V>>),
        );
        app.add_systems(
            PostUpdate,
            Self::poll_logins::<V>.run_if(any_with_component::<LoginTask<V>>),
        );
    }

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

    /// The default reason for denying a connection.
    const DEFAULT_REASON: &str = "Connection refused";

    /// Poll [`LoginTask`]s for completion.
    ///
    /// Sends a [`ConnectionLoginEvent`] when a task completes.
    #[expect(clippy::missing_panics_doc)]
    #[expect(clippy::type_complexity)]
    pub fn poll_logins<V: Version>(
        world: &mut World,
        mut cache: Local<Vec<(Entity, Connection<V, Configuration, Clientbound>)>>,
    ) where
        Clientbound: NetworkDirection<V, Login> + NetworkDirection<V, Configuration>,
        Login: State<V>,
        Configuration: State<V>,
    {
        let mut state =
            SystemState::<(Query<(Entity, &GameProfile, &mut LoginTask<V>)>, Commands)>::new(world);
        let (mut query, mut commands) = state.get_mut(world);

        for (entity, profile, mut task) in &mut query {
            match task.poll() {
                Some(Ok(connection)) => {
                    cache.push((entity, connection));
                    commands.entity(entity).remove::<LoginTask<V>>();
                }
                Some(Err(error)) => {
                    error!(target: TARGET, "Failed to log in {}: {error}", profile.name);

                    debug!(target: TARGET, "Despawning {}'s Entity {}", profile.name, entity);
                    commands.entity(entity).despawn_recursive();
                }
                None => {}
            }
        }

        state.apply(world);

        for (entity, connection) in cache.drain(..) {
            let profile = world.get::<GameProfile>(entity).expect("Missing GameProfile");
            match world.resource::<LoginChecklist<V>>().check(entity, world) {
                LoginAction::Accept => {
                    info!(target: TARGET, "Accepted login from {}", profile.name);

                    let connection = Mutex::new(Some(connection));
                    world.send_event(ConnectionLoginEvent { entity, connection });
                }
                LoginAction::Deny(reason) => {
                    let reason = reason.unwrap_or(Self::DEFAULT_REASON.to_string());
                    warn!(target: TARGET, "Denied login from {}: {reason}", profile.name);

                    debug!(target: TARGET, "Despawning {}'s Entity {}", profile.name, entity);
                    world.commands().entity(entity).despawn_recursive();
                }
            }
        }
    }

    /// Receive [`Login`] packets from [`LoginTask`]s.
    ///
    /// Sends [`LoginPacketEvent`]s when packets are received.
    pub fn login_packets<V: Version>(
        query: Query<(Entity, &LoginTask<V>)>,
        mut events: EventWriter<LoginPacketEvent<V>>,
    ) where
        Clientbound: NetworkDirection<V, Login> + NetworkDirection<V, Configuration>,
        Login: State<V>,
        Configuration: State<V>,
    {
        for (entity, task) in &query {
            while let Some(packet) = task.recv() {
                events.send(LoginPacketEvent { entity, packet });
            }
        }
    }
}
