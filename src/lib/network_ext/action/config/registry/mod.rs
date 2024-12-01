use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use super::RequiredFinishComponents;
use crate::{
    network::{ConfigFilter, ConfigTask, FilterResult},
    network_ext::{NetworkExtConfigSet, TARGET},
    world::DimensionList,
};

mod v1_21_0;

/// A [`Plugin`] that adds sending registry packets to clients.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfigRegistryPlugin<V: Version>(PhantomData<V>);

impl<V: Version + ConfigRegistryTrait> Plugin for ConfigRegistryPlugin<V>
where
    Clientbound: NetworkDirection<V, Configuration>,
    Configuration: State<V>,
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::send_registries.in_set(NetworkExtConfigSet));
    }

    fn finish(&self, app: &mut App) {
        let mut filters = app.world_mut().resource_mut::<ConfigFilter<V>>();
        filters.add_filter(Self::require_configuration_packets);

        let mut required = app.world_mut().resource_mut::<RequiredFinishComponents<V>>();
        required.add::<WasSentRegistries>();
    }
}

/// A [`Component`] that marks registry packets as already sent.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
#[component(storage = "SparseSet")]
pub struct WasSentRegistries;

impl<V: Version + ConfigRegistryTrait> ConfigRegistryPlugin<V>
where
    Clientbound: NetworkDirection<V, Configuration>,
    Configuration: State<V>,
{
    /// A system that sends clients registry packets.
    pub fn send_registries(
        query: Query<(Entity, &GameProfile, &ConfigTask<V>), Without<WasSentRegistries>>,
        dimensions: Res<DimensionList>,
        mut commands: Commands,
    ) {
        for (entity, profile, task) in &query {
            debug!(target: TARGET, "Sending registries to {}", profile.name);
            V::send_registries(&dimensions, task);
            commands.entity(entity).insert(WasSentRegistries);
        }
    }

    const DENY_REASON: &'static str = "Registries not sent";

    /// A filter that denies clients that
    /// have not received any registry packets.
    fn require_configuration_packets(entity: Entity, world: &World) -> FilterResult {
        if world.get::<WasSentRegistries>(entity).is_some() {
            FilterResult::Allow
        } else {
            FilterResult::Deny(Some(Self::DENY_REASON.into()))
        }
    }
}

/// A trait for sending registry packets to clients.
pub trait ConfigRegistryTrait: Version
where
    Clientbound: NetworkDirection<Self, Configuration>,
    Configuration: State<Self>,
{
    /// Send registry packets to a client.
    fn send_registries(dimensions: &DimensionList, task: &ConfigTask<Self>);
}
