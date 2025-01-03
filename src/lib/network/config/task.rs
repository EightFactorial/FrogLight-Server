use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};
use parking_lot::Mutex;

use super::{
    CompletedConfig, ConfigPacketEvent, ConfigRegistryTrait, ConfigRequiredComponents, ConfigTask,
    ConfigTrait, HasRegistries,
};
use crate::network::{common::channel, config::ConfigStateEvent, login::LoginStateEvent};

impl<V: Version + ConfigTrait + ConfigRegistryTrait> ConfigTask<V>
where
    Clientbound: NetworkDirection<V, Login> + NetworkDirection<V, Configuration>,
    Login: State<V>,
    Configuration: State<V>,
{
    /// A system that configures incoming logins.
    #[expect(clippy::missing_panics_doc)]
    pub fn receive_logins(
        query: Query<&GameProfile>,
        mut events: EventReader<LoginStateEvent<V>>,
        mut commands: Commands,
    ) {
        for LoginStateEvent { entity, connection } in events.read() {
            if let Some(conn) = connection.lock().take() {
                debug!("Configuring {} ...", query.get(*entity).unwrap().username);
                commands.entity(*entity).insert(ConfigTask::new(conn.configuration()));
            }
        }
    }
}

impl<V: Version + ConfigTrait + ConfigRegistryTrait> ConfigTask<V>
where
    Clientbound: NetworkDirection<V, Configuration>,
    Configuration: State<V>,
{
    /// Create a new [`ConfigTask`] with the given [`Connection`]
    #[must_use]
    pub fn new(conn: Connection<V, Configuration, Clientbound>) -> Self {
        let (send, recv) = channel();
        Self::spawn(send, V::config(conn, recv))
    }

    /// A system that sends registries to clients that
    /// have not received them yet.
    pub fn send_registries(
        query: Query<(Entity, &GameProfile, &ConfigTask<V>), Without<HasRegistries>>,
        mut commands: Commands,
    ) {
        for (entity, profile, task) in &query {
            debug!("Sending registries to {}", profile.username);
            V::send_registries(task);
            commands.entity(entity).insert(HasRegistries);
        }
    }

    /// A system that completes all configurations that have the required
    /// components.
    pub fn complete_configurations(
        query: Query<(Entity, &GameProfile, &ConfigTask<V>), Without<CompletedConfig>>,
        required: Res<ConfigRequiredComponents<V>>,
        world: &World,
        mut commands: Commands,
    ) {
        for (entity, profile, task) in &query {
            if required.check(entity, world) {
                debug!("Sending ready to {}", profile.username);
                V::send_finish(task);
                commands.entity(entity).insert(CompletedConfig);
            }
        }
    }
}

impl<V: Version> ConfigTask<V>
where
    Clientbound: NetworkDirection<V, Configuration>,
    Configuration: State<V>,
{
    /// A system that receives packets from all configuration tasks.
    pub fn receive_packets(
        query: Query<(Entity, &ConfigTask<V>)>,
        mut events: EventWriter<ConfigPacketEvent<V>>,
    ) {
        for (entity, task) in &query {
            while let Some(packet) = task.recv() {
                events.send(ConfigPacketEvent::new(entity, packet));
            }
        }
    }

    /// A system that polls all configuration tasks and
    /// despawns them if they are done.
    pub fn poll_tasks(
        mut query: Query<(Entity, &GameProfile, &mut ConfigTask<V>)>,
        mut events: EventWriter<ConfigStateEvent<V>>,
        mut commands: Commands,
    ) {
        for (entity, profile, mut task) in &mut query {
            match task.poll() {
                Some(Ok(conn)) => {
                    debug!("Configured {}", profile.username);
                    commands.entity(entity).remove::<ConfigTask<V>>();
                    events.send(ConfigStateEvent { entity, connection: Mutex::new(Some(conn)) });
                }
                Some(Err(err)) => {
                    error!("Configuration failed for {}: {err}", profile.username);
                    debug!("Despawning Entity {entity}");
                    commands.entity(entity).despawn_recursive();
                }
                None => {}
            }
        }
    }
}
