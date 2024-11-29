use bevy::{ecs::system::SystemState, prelude::*};
use compact_str::ToCompactString;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use super::{version::ConfigTrait, ConfigFilter, ConfigPacketEvent, ConfigTask};
use crate::network::{config::TARGET, ConfigStateEvent, FilterResult, LoginStateEvent};

impl<V: Version> ConfigTask<V>
where
    Clientbound: NetworkDirection<V, Configuration>,
    Configuration: State<V>,
{
    pub(super) fn poll_packets(
        mut query: Query<(Entity, &ConfigTask<V>)>,
        mut events: EventWriter<ConfigPacketEvent<V>>,
    ) {
        for (entity, listener) in &mut query {
            while let Some(packet) = listener.recv() {
                events.send(ConfigPacketEvent { entity, packet });
            }
        }
    }
}

impl<V: Version + ConfigTrait> ConfigTask<V>
where
    Clientbound: NetworkDirection<V, Login> + NetworkDirection<V, Configuration>,
    Login: State<V>,
    Configuration: State<V>,
{
    pub(super) fn receive_logins(
        query: Query<&GameProfile>,
        mut events: EventReader<LoginStateEvent<V>>,
        mut commands: Commands,
    ) {
        for event in events.read() {
            if let Some(conn) = event.take() {
                commands.entity(event.entity).insert(V::new_config(conn.configuration()));
            } else if let Ok(profile) = query.get(event.entity) {
                error!(target: TARGET, "Failed to receive connection for {}", profile.name);
            } else {
                error!(target: TARGET, "Failed to receive connection for {}", event.entity);
            }
        }
    }
}

impl<V: Version> ConfigTask<V>
where
    Clientbound: NetworkDirection<V, Configuration>,
    Configuration: State<V>,
{
    const DEFAULT_REASON: &'static str = "Denied by server";

    #[expect(clippy::type_complexity)]
    pub(super) fn poll_tasks(
        world: &mut World,
        mut cache: Local<Vec<(Entity, Connection<V, Configuration, Clientbound>)>>,
    ) {
        let mut state = SystemState::<(
            Query<(Entity, Option<&GameProfile>, &mut ConfigTask<V>)>,
            Commands,
        )>::new(world);
        let (mut query, mut commands) = state.get_mut(world);

        for (entity, profile, mut listener) in &mut query {
            match listener.poll() {
                Some(Ok(conn)) => {
                    cache.push((entity, conn));
                    commands.entity(entity).remove::<ConfigTask<V>>();
                }
                Some(Err(err)) => {
                    if let Some(profile) = profile {
                        error!(target: TARGET, "Failed to configure {}: {err}", profile.name);
                    } else {
                        error!(target: TARGET, "Failed to configure {entity}: {err}");
                    }

                    debug!(target: TARGET, "Despawning Entity {entity}");
                    commands.entity(entity).despawn_recursive();
                }
                None => {}
            }
        }

        state.apply(world);

        for (entity, conn) in cache.drain(..) {
            match world.resource::<ConfigFilter<V>>().filter(entity, world) {
                FilterResult::Allow => {
                    let profile = world.get::<GameProfile>(entity).unwrap();
                    info!(target: TARGET, "Successfully configured {}", profile.name);
                    world.send_event(ConfigStateEvent::<V>::new(entity, conn));
                }
                FilterResult::Deny(reason) => {
                    let reason = reason.unwrap_or(Self::DEFAULT_REASON.to_compact_string());
                    if let Some(profile) = world.get::<GameProfile>(entity) {
                        error!(target: TARGET, "Refused to configure {}: {reason}", profile.name);
                    } else {
                        error!(target: TARGET, "Refused to configure {entity}: {reason}");
                    }

                    debug!(target: TARGET, "Despawning Entity {entity}");
                    world.entity_mut(entity).despawn_recursive();
                }
            }
        }
    }
}
