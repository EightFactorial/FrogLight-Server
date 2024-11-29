use bevy::{ecs::system::SystemState, prelude::*, utils::HashMap};
use compact_str::ToCompactString;
use froglight::{
    network::connection::{AccountInformation, NetworkDirection},
    prelude::{State, *},
};

use super::{ConnectionInfo, LoginFilter, LoginPacketEvent, LoginTask, LoginTrait};
use crate::network::{login::TARGET, ConnectionRequestEvent, FilterResult};

impl<V: Version> LoginTask<V>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    pub(super) fn poll_packets(
        mut query: Query<(Entity, &LoginTask<V>)>,
        mut events: EventWriter<LoginPacketEvent<V>>,
    ) {
        for (entity, listener) in &mut query {
            while let Some(packet) = listener.recv() {
                events.send(LoginPacketEvent { entity, packet });
            }
        }
    }
}

impl<V: Version + LoginTrait> LoginTask<V>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    pub(super) fn receive_requests(
        mut events: EventReader<ConnectionRequestEvent<V>>,
        mut commands: Commands,
    ) {
        for event in events.read() {
            let Some(conn) = event.request.take() else {
                error!(target: TARGET, "Failed to receive connection from {}", event.request.username);
                continue;
            };

            let info = ConnectionInfo::from(&event.request);

            let uuid = AccountInformation::offline_uuid(&event.request.username);
            let profile = GameProfile {
                uuid,
                name: event.request.username.clone(),
                properties: HashMap::new(),
            };

            let entity = commands.spawn((info, profile, V::new_login(conn)));
            debug!(target: TARGET, "Spawning Entity {} for {}", entity.id(), event.request.username);
        }
    }
}

impl<V: Version> LoginTask<V>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    const DEFAULT_REASON: &'static str = "Denied by server";

    #[expect(clippy::type_complexity)]
    pub(super) fn poll_tasks(
        world: &mut World,
        mut cache: Local<Vec<(Entity, Connection<V, Login, Clientbound>)>>,
    ) {
        let mut state = SystemState::<(
            Query<(Entity, Option<&GameProfile>, &mut LoginTask<V>)>,
            Commands,
        )>::new(world);
        let (mut query, mut commands) = state.get_mut(world);

        for (entity, profile, mut listener) in &mut query {
            match listener.poll() {
                Some(Ok(conn)) => {
                    cache.push((entity, conn));
                    commands.entity(entity).remove::<LoginTask<V>>();
                }
                Some(Err(err)) => {
                    if let Some(profile) = profile {
                        error!(target: TARGET, "Failed to login {}: {err}", profile.name);
                    } else {
                        error!(target: TARGET, "Failed to login {entity}: {err}");
                    }

                    debug!(target: TARGET, "Despawning Entity {entity}");
                    commands.entity(entity).despawn_recursive();
                }
                None => {}
            }
        }

        state.apply(world);

        for (entity, _conn) in cache.drain(..) {
            match world.resource::<LoginFilter<V>>().filter(entity, world) {
                FilterResult::Allow => {
                    let profile = world.get::<GameProfile>(entity).unwrap();
                    info!(target: TARGET, "Successfully logged in {}", profile.name);
                }
                FilterResult::Deny(reason) => {
                    let reason = reason.unwrap_or(Self::DEFAULT_REASON.to_compact_string());
                    if let Some(profile) = world.get::<GameProfile>(entity) {
                        error!(target: TARGET, "Refused to login {}: {reason}", profile.name);
                    } else {
                        error!(target: TARGET, "Refused to login {entity}: {reason}");
                    }

                    debug!(target: TARGET, "Despawning Entity {entity}");
                    world.entity_mut(entity).despawn_recursive();
                }
            }
        }
    }
}
