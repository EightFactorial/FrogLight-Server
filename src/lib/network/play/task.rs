use bevy::{ecs::system::SystemState, prelude::*};
use compact_str::ToCompactString;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{entity::Player, State, *},
};

use super::{version::PlayTrait, PlayFilter, PlayPacketEvent, PlayTask};
use crate::network::{play::TARGET, ConfigStateEvent, FilterResult, PlayStateEvent};

impl<V: Version> PlayTask<V>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    pub(super) fn poll_packets(
        mut query: Query<(Entity, &PlayTask<V>)>,
        mut events: EventWriter<PlayPacketEvent<V>>,
    ) {
        for (entity, listener) in &mut query {
            while let Some(packet) = listener.recv() {
                events.send(PlayPacketEvent { entity, packet });
            }
        }
    }
}

impl<V: Version + PlayTrait> PlayTask<V>
where
    Clientbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    pub(super) fn receive_configured(
        query: Query<&GameProfile>,
        mut events: EventReader<ConfigStateEvent<V>>,
        mut commands: Commands,
    ) {
        for event in events.read() {
            if let Some(conn) = event.take() {
                commands.entity(event.entity).insert((V::new_play(conn.play()), Player));
            } else if let Ok(profile) = query.get(event.entity) {
                error!(target: TARGET, "Failed to receive connection for {}", profile.name);
            } else {
                error!(target: TARGET, "Failed to receive connection for {}", event.entity);
            }
        }
    }
}

impl<V: Version> PlayTask<V>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    const DEFAULT_REASON: &'static str = "Denied by server";

    #[expect(clippy::type_complexity)]
    pub(super) fn poll_tasks(
        world: &mut World,
        mut cache: Local<Vec<(Entity, Connection<V, Play, Clientbound>)>>,
    ) {
        let mut state = SystemState::<(
            Query<(Entity, Option<&GameProfile>, &mut PlayTask<V>)>,
            Commands,
        )>::new(world);
        let (mut query, mut commands) = state.get_mut(world);

        for (entity, profile, mut listener) in &mut query {
            match listener.poll() {
                Some(Ok(conn)) => {
                    cache.push((entity, conn));
                    commands.entity(entity).remove::<PlayTask<V>>();
                }
                Some(Err(err)) => {
                    if let Some(profile) = profile {
                        if let ConnectionError::ConnectionClosed = err {
                            info!(target: TARGET, "{} disconnected", profile.name);
                        } else {
                            error!(target: TARGET, "{} disconnected: {err}", profile.name);
                        }
                    } else if let ConnectionError::ConnectionClosed = err {
                        warn!(target: TARGET, "Entity {entity} disconnected");
                    } else {
                        error!(target: TARGET, "Entity {entity} disconnected: {err}");
                    }

                    debug!(target: TARGET, "Despawning Entity {entity}");
                    commands.entity(entity).despawn_recursive();
                }
                None => {}
            }
        }

        state.apply(world);

        for (entity, conn) in cache.drain(..) {
            match world.resource::<PlayFilter<V>>().filter(entity, world) {
                FilterResult::Allow => {
                    let profile = world.get::<GameProfile>(entity).unwrap();
                    info!(target: TARGET, "Reconfiguring {}", profile.name);
                    world.send_event(PlayStateEvent::<V>::new(entity, conn));
                }
                FilterResult::Deny(reason) => {
                    let reason = reason.unwrap_or(Self::DEFAULT_REASON.to_compact_string());
                    if let Some(profile) = world.get::<GameProfile>(entity) {
                        error!(target: TARGET, "Refused to reconfigure {}: {reason}", profile.name);
                    } else {
                        error!(target: TARGET, "Refused to reconfigure {entity}: {reason}");
                    }

                    debug!(target: TARGET, "Despawning Entity {entity}");
                    world.entity_mut(entity).despawn_recursive();
                }
            }
        }
    }
}
