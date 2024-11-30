use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use super::{ClientConfiguration, ClientKnownPacks, WasSentRegistries};
use crate::{
    network::{ConfigFilter, ConfigTask, FilterResult},
    network_ext::{NetworkExtSystemSet, TARGET},
};

mod v1_21_0;

/// A [`Plugin`] that finishes the configuration process.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfigFinishPlugin<V: Version>(PhantomData<V>);

impl<V: Version + ConfigFinishTrait> Plugin for ConfigFinishPlugin<V>
where
    Clientbound: NetworkDirection<V, Configuration>,
    Configuration: State<V>,
{
    fn build(&self, app: &mut App) {
        let mut filters = app.world_mut().resource_mut::<ConfigFilter<V>>();
        filters.add_filter(Self::require_finish_packet);

        app.add_systems(
            Update,
            (
                Self::send_finish.run_if(any_with_component::<ConfigTask<V>>),
                Self::remove_finish.run_if(any_component_removed::<ConfigTask<V>>),
            )
                .in_set(NetworkExtSystemSet),
        );
    }
}

/// A [`Component`] that marks the finish packet as already sent.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
#[component(storage = "SparseSet")]
pub struct WasSentFinish;

impl<V: Version + ConfigFinishTrait> ConfigFinishPlugin<V>
where
    Clientbound: NetworkDirection<V, Configuration>,
    Configuration: State<V>,
{
    /// A system that sends the finish packet to clients.
    #[expect(clippy::type_complexity)]
    pub fn send_finish(
        query: Query<
            (Entity, &GameProfile, &ConfigTask<V>),
            (
                With<ClientConfiguration>,
                With<ClientKnownPacks>,
                With<WasSentRegistries>,
                Without<WasSentFinish>,
            ),
        >,
        mut commands: Commands,
    ) {
        for (entity, profile, task) in &query {
            debug!(target: TARGET, "Sending finish to {}", profile.name);
            V::send_finish(task);
            commands.entity(entity).insert(WasSentFinish);
        }
    }

    /// A system that removes the finish marker from clients.
    pub fn remove_finish(
        query: Query<(Entity, &GameProfile), With<WasSentFinish>>,
        mut commands: Commands,
    ) {
        for (entity, profile) in &query {
            debug!(target: TARGET, "Removing finish flag from {}", profile.name);
            commands.entity(entity).remove::<WasSentFinish>();
        }
    }

    const DENY_REASON: &'static str = "Finish not sent by server";

    /// A filter that denies clients that
    /// have not ben sent a finish packet.
    fn require_finish_packet(entity: Entity, world: &World) -> FilterResult {
        if world.get::<WasSentFinish>(entity).is_some() {
            FilterResult::Allow
        } else {
            FilterResult::Deny(Some(Self::DENY_REASON.into()))
        }
    }
}

/// A trait for finishing the configuration process.
pub trait ConfigFinishTrait: Version
where
    Clientbound: NetworkDirection<Self, Configuration>,
    Configuration: State<Self>,
{
    /// Send a finish packet to a client.
    fn send_finish(task: &ConfigTask<Self>);
}
