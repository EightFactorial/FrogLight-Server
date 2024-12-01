use std::{any::TypeId, marker::PhantomData};

use bevy::{prelude::*, utils::HashSet};
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use crate::{
    network::{ConfigFilter, ConfigTask, FilterResult},
    network_ext::{NetworkExtConfigSet, NetworkExtPlaySet, TARGET},
};

mod v1_21_0;

/// A [`Plugin`] that finishes the configuration process.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfigFinishPlugin<V: Version>(PhantomData<V>);

/// A [`Resource`] that stores the required [`Components`](Component)
/// for connections to finish [`Configuration`].
#[derive(Debug, Default, Clone, PartialEq, Eq, Resource)]
pub struct RequiredFinishComponents<V: Version> {
    components: HashSet<TypeId>,
    _phantom: PhantomData<V>,
}

impl<V: Version> RequiredFinishComponents<V> {
    /// Adds a [`Component`] to the list of required components.
    ///
    /// Returns `true` if the component was new,
    /// `false` if it was already required.
    pub fn add<T: Component>(&mut self) -> bool { self.components.insert(TypeId::of::<T>()) }

    /// Returns `true` if the component is required.
    #[must_use]
    pub fn contains<T: Component>(&self) -> bool { self.contains_type(&TypeId::of::<T>()) }

    /// Returns `true` if the component is required.
    #[must_use]
    pub fn contains_type(&self, type_id: &TypeId) -> bool { self.components.contains(type_id) }
}

impl<V: Version + ConfigFinishTrait> Plugin for ConfigFinishPlugin<V>
where
    Clientbound: NetworkDirection<V, Configuration>,
    Configuration: State<V>,
{
    fn build(&self, app: &mut App) {
        app.init_resource::<RequiredFinishComponents<V>>();

        app.add_systems(
            Update,
            (
                Self::send_finish_packet.in_set(NetworkExtConfigSet),
                Self::remove_finish_marker
                    .run_if(any_component_removed::<ConfigTask<V>>)
                    .in_set(NetworkExtPlaySet),
            ),
        );
    }

    fn finish(&self, app: &mut App) {
        let mut filters = app.world_mut().resource_mut::<ConfigFilter<V>>();
        filters.add_filter(Self::require_finish_packet);
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
    pub fn send_finish_packet(
        world: &World,
        query: Query<(Entity, &GameProfile, &ConfigTask<V>), Without<WasSentFinish>>,
        required: Res<RequiredFinishComponents<V>>,
        mut commands: Commands,
    ) {
        for (entity, profile, task) in &query {
            let required_count = required.components.len();
            if world
                .inspect_entity(entity)
                .filter(|c| c.type_id().is_some_and(|c| required.contains_type(&c)))
                .count()
                == required_count
            {
                debug!(target: TARGET, "Sending finish to {}", profile.name);
                V::send_finish(task);
                commands.entity(entity).insert(WasSentFinish);
            }
        }
    }

    /// A system that removes the finish marker from clients.
    pub fn remove_finish_marker(
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
