use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use crate::{
    configuration::{ConfigTask, TARGET},
    ConfigAction, ConfigPacketEvent,
};

mod v1_21_0;

/// A list of known packs by the server.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub struct KnownPacks<V: Version> {
    resourcepacks: Vec<KnownResourcePack>,
    _phantom: PhantomData<V>,
}

impl<V: Version + KnownPacksConfig> Default for KnownPacks<V>
where
    Clientbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    fn default() -> Self {
        Self {
            resourcepacks: <V as KnownPacksConfig>::DEFAULT_PACKS.to_vec(),
            _phantom: PhantomData,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Component)]
#[component(storage = "Table")]
pub(crate) struct HasKnownPacks;

/// A list of known packs sent by the client.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Component)]
pub struct ClientKnownPacks(Vec<KnownResourcePack>);

#[expect(private_bounds)]
impl<V: Version + KnownPacksConfig> KnownPacks<V>
where
    Clientbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    /// When a client connects, send the known packs to the client.
    #[expect(clippy::type_complexity, private_interfaces)]
    pub fn send_known_packs(
        query: Query<(Entity, &ConfigTask<V>), (Added<ConfigTask<V>>, Without<HasKnownPacks>)>,
        packs: Res<KnownPacks<V>>,
        mut commands: Commands,
    ) {
        for (entity, task) in &query {
            trace!(target: TARGET, "Sending known packs: {:?}", packs.resourcepacks);

            <V as KnownPacksConfig>::send_packs(task, &packs);
            commands.entity(entity).insert(HasKnownPacks);
        }
    }

    /// A system that receives known packs from the client.
    pub fn recv_known_packs(
        query: Query<&GameProfile>,
        mut events: EventReader<ConfigPacketEvent<V>>,
        mut commands: Commands,
    ) {
        for event in events.read() {
            if let Some(packs) = <V as KnownPacksConfig>::recv_packs(&event.packet) {
                if let Ok(profile) = query.get(event.entity) {
                    debug!(target: TARGET, "Received known packs from {}: {:?}", profile.name, packs.0);
                } else {
                    warn!(target: TARGET, "Received known packs for: {}", event.entity);
                }

                commands.entity(event.entity).insert(packs);
            }
        }
    }

    const DENY_REASON: &'static str = "Client has not sent known packs";

    /// A [`ConfigChecklist`](crate::configuration::ConfigChecklist) function
    /// that checks if the client has replied with known packs.
    pub fn has_known_packs(entity: Entity, world: &World) -> ConfigAction {
        if world.get::<HasKnownPacks>(entity).is_some() {
            ConfigAction::Accept
        } else {
            ConfigAction::Deny(Some(Self::DENY_REASON.to_string()))
        }
    }
}

pub(crate) trait KnownPacksConfig: Version
where
    Clientbound: NetworkDirection<Self, Configuration> + NetworkDirection<Self, Play>,
    Configuration: State<Self>,
    Play: State<Self>,
{
    const DEFAULT_PACKS: &'static [KnownResourcePack];

    fn send_packs(task: &ConfigTask<Self>, packs: &KnownPacks<Self>);

    fn recv_packs(
        packet: &<Configuration as State<Self>>::ServerboundPacket,
    ) -> Option<ClientKnownPacks>;
}
