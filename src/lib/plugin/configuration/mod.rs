//! TODO

use bevy::{ecs::system::SystemState, prelude::*};
use froglight::{
    network::{connection::NetworkDirection, versions::v1_21_0::V1_21_0},
    prelude::{State, *},
};

mod action;
pub use action::*;

mod channel;
pub use channel::{channel, AsyncConfigChannel, TaskConfigChannel};

mod checklist;
pub use checklist::{ConfigAction, ConfigChecklist};

mod event;
pub use event::{ConfigPacketEvent, ConnectionConfigEvent};

mod task;
use parking_lot::Mutex;
pub use task::ConfigTask;
use task::ConnectionConfig;

use super::login::ConnectionLoginEvent;

/// A plugin that manages connection configuration.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ServerBrand>();

        Self::add_version::<V1_21_0>(app);
    }
}

static TARGET: &str = "CONFG";

impl ConfigPlugin {
    /// Add systems, resources, and events for the given [`Version`].
    fn add_version<V: Version + ConnectionConfig + SendServerBrand>(app: &mut App)
    where
        Clientbound: NetworkDirection<V, Login>
            + NetworkDirection<V, Configuration>
            + NetworkDirection<V, Play>,
        Login: State<V>,
        Configuration: State<V>,
        Play: State<V>,
    {
        app.add_event::<ConnectionConfigEvent<V>>();
        app.add_event::<ConfigPacketEvent<V>>();

        app.init_resource::<ConfigChecklist<V>>();

        app.add_systems(
            PreUpdate,
            Self::config_packets::<V>.run_if(any_with_component::<ConfigTask<V>>),
        );

        app.add_systems(
            Update,
            (ServerBrand::send_server_brand::<V>, ServerBrand::recv_client_brand::<V>)
                .run_if(any_with_component::<ConfigTask<V>>),
        );

        app.add_systems(
            PostUpdate,
            (
                Self::spawn_configs::<V>.run_if(on_event::<ConnectionLoginEvent<V>>),
                Self::poll_configs::<V>.run_if(any_with_component::<ConfigTask<V>>),
            )
                .chain(),
        );
    }

    fn spawn_configs<V: Version + ConnectionConfig>(
        query: Query<&GameProfile>,
        mut events: EventReader<ConnectionLoginEvent<V>>,
        mut commands: Commands,
    ) where
        Clientbound: NetworkDirection<V, Login>
            + NetworkDirection<V, Configuration>
            + NetworkDirection<V, Play>,
        Login: State<V>,
        Configuration: State<V>,
        Play: State<V>,
    {
        for event in events.read() {
            if let Some(connection) = std::mem::take(&mut *event.connection.lock()) {
                if let Ok(profile) = query.get(event.entity) {
                    info!(target: TARGET, "Starting configuration for {}", profile.name);
                } else {
                    info!(target: TARGET, "Starting configuration for {}", event.entity);
                }

                commands.entity(event.entity).insert(ConfigTask::spawn(connection));
            } else {
                error!(target: TARGET, "Connection was stolen before configuration");

                debug!(target: TARGET, "Despawning Entity {}", event.entity);
                commands.entity(event.entity).despawn_recursive();
            }
        }
    }

    /// The default reason for denying a connection's configuration.
    const DEFAULT_REASON: &str = "Invalid configuration";

    /// Poll [`ConfigTask`]s for completion.
    ///
    /// Sends a [`ConnectionConfigEvent`] when a task completes.
    #[expect(clippy::missing_panics_doc)]
    #[expect(clippy::type_complexity)]
    pub fn poll_configs<V: Version>(
        world: &mut World,
        mut cache: Local<Vec<(Entity, Connection<V, Play, Clientbound>)>>,
    ) where
        Clientbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
        Configuration: State<V>,
        Play: State<V>,
    {
        let mut state =
            SystemState::<(Query<(Entity, &GameProfile, &mut ConfigTask<V>)>, Commands)>::new(
                world,
            );
        let (mut query, mut commands) = state.get_mut(world);

        for (entity, profile, mut task) in &mut query {
            match task.poll() {
                Some(Ok(connection)) => {
                    cache.push((entity, connection));
                    commands.entity(entity).remove::<ConfigTask<V>>();
                }
                Some(Err(error)) => {
                    error!(target: TARGET, "Failed to configure {}: {error}", profile.name);

                    debug!(target: TARGET, "Despawning {}'s Entity {}", profile.name, entity);
                    commands.entity(entity).despawn_recursive();
                }
                None => {}
            }
        }

        state.apply(world);

        for (entity, connection) in cache.drain(..) {
            let profile = world.get::<GameProfile>(entity).expect("Missing GameProfile");
            match world.resource::<ConfigChecklist<V>>().check(entity, world) {
                ConfigAction::Accept => {
                    info!(target: TARGET, "Accepted confuration from {}", profile.name);

                    let connection = Mutex::new(Some(connection));
                    world.send_event(ConnectionConfigEvent { entity, connection });
                }
                ConfigAction::Deny(reason) => {
                    let reason = reason.unwrap_or(Self::DEFAULT_REASON.to_string());
                    warn!(target: TARGET, "Denied confuration from {}: {reason}", profile.name);

                    debug!(target: TARGET, "Despawning {}'s Entity {}", profile.name, entity);
                    world.commands().entity(entity).despawn_recursive();
                }
            }
        }
    }

    /// Receive [`Configuration`] packets from [`ConfigTask`]s.
    ///
    /// Sends [`ConfigPacketEvent`]s when packets are received.
    pub fn config_packets<V: Version>(
        query: Query<(Entity, &ConfigTask<V>)>,
        mut events: EventWriter<ConfigPacketEvent<V>>,
    ) where
        Clientbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
        Configuration: State<V>,
        Play: State<V>,
    {
        for (entity, task) in &query {
            while let Some(packet) = task.recv() {
                events.send(ConfigPacketEvent { entity, packet });
            }
        }
    }
}
