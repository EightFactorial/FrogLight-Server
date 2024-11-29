use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use crate::{
    network::{ConfigFilter, ConfigPacketEvent, ConfigTask, FilterResult},
    network_ext::{NetworkExtSystemSet, TARGET},
};

mod v1_21_0;

/// A [`Plugin`] that adds sending and receiving
/// known resourcepack packets to clients.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfigKnownPackPlugin<V: Version>(PhantomData<V>);

impl<V: Version + ConfigPacketTrait> Plugin for ConfigKnownPackPlugin<V>
where
    Clientbound: NetworkDirection<V, Configuration>,
    Configuration: State<V>,
{
    fn build(&self, app: &mut App) {
        let mut filters = app.world_mut().resource_mut::<ConfigFilter<V>>();
        filters.add_filter(Self::require_resourcepacks);

        app.add_systems(
            Update,
            (Self::send_known_packs, Self::receive_known_packs)
                .run_if(any_with_component::<ConfigTask<V>>)
                .in_set(NetworkExtSystemSet),
        );
    }
}

/// A [`Component`] that marks a [`Configuration`] as already sent.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct WasSentPacks;

/// A [`Component`] that holds the client's known resourcepacks.
#[derive(Debug, Default, Clone, PartialEq, Hash, Deref, DerefMut, Component)]
pub struct ClientKnownPacks(Vec<KnownResourcePack>);

impl<V: Version + ConfigPacketTrait> ConfigKnownPackPlugin<V>
where
    Clientbound: NetworkDirection<V, Configuration>,
    Configuration: State<V>,
{
    /// A system that sends known resourcepacks to clients.
    pub fn send_known_packs(
        query: Query<(Entity, &GameProfile, &ConfigTask<V>), Without<WasSentPacks>>,
        mut commands: Commands,
    ) {
        for (entity, profile, task) in &query {
            debug!(target: TARGET, "Sending known resource packs to {}", profile.name);
            V::send_packs(task);
            commands.entity(entity).insert(WasSentPacks);
        }
    }

    /// A system that receives known resourcepacks from clients.
    pub fn receive_known_packs(
        mut query: Query<(&GameProfile, Option<&mut ClientKnownPacks>)>,
        mut events: EventReader<ConfigPacketEvent<V>>,
        mut commands: Commands,
    ) {
        for event in events.read() {
            if let Some(new_packs) = V::receive_packs(&event.packet) {
                if let Ok((profile, known_packs)) = query.get_mut(event.entity) {
                    if let Some(mut known_packs) = known_packs {
                        if *known_packs != new_packs {
                            warn!(target: TARGET, "Received different known resource packs from {}", profile.name);
                        }
                        *known_packs = new_packs;
                    } else {
                        debug!(target: TARGET, "Received known resource packs from {}", profile.name);
                        commands.entity(event.entity).insert(new_packs);
                    }
                }
            }
        }
    }

    const DENY_REASON: &'static str = "Known resource packs not received";

    /// A filter that denies clients that
    /// have not replied with known resourcepacks.
    fn require_resourcepacks(entity: Entity, world: &World) -> FilterResult {
        if world.get::<ClientKnownPacks>(entity).is_some() {
            FilterResult::Allow
        } else {
            FilterResult::Deny(Some(Self::DENY_REASON.into()))
        }
    }
}

/// A trait for sending configuration packets to a client.
pub trait ConfigPacketTrait: Version
where
    Clientbound: NetworkDirection<Self, Configuration>,
    Configuration: State<Self>,
{
    /// Send configuration packets to a client.
    fn send_packs(task: &ConfigTask<Self>);

    /// Receive known resourcepacks from a client.
    fn receive_packs(
        packet: &<Configuration as State<Self>>::ServerboundPacket,
    ) -> Option<ClientKnownPacks>;
}
