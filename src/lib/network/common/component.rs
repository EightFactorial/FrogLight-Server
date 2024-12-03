use std::{any::TypeId, hash::BuildHasherDefault, marker::PhantomData};

use bevy::{
    prelude::{Component, Entity, EntityRef, Resource, World},
    utils::HashSet,
};
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

/// A list of required [`Component`]s to meet a condition.
#[derive(Debug, Default, Resource)]
pub struct ComponentFilter<V: Version, S: State<V>>
where
    Clientbound: NetworkDirection<V, S>,
{
    components: HashSet<TypeId>,
    _phantom: PhantomData<(V, S)>,
}

impl<V: Version, S: State<V>> ComponentFilter<V, S>
where
    Clientbound: NetworkDirection<V, S>,
{
    /// Create a new empty [`ConnectionComponents`].
    #[must_use]
    pub const fn new_empty() -> Self {
        Self { components: HashSet::with_hasher(BuildHasherDefault::new()), _phantom: PhantomData }
    }

    /// Add a [`Component`] to the list of required [`Component`]s.
    ///
    /// Returns `true` if the [`Component`] was not already in the list.
    #[inline]
    pub fn add_required<T: Component>(&mut self) -> bool {
        self.add_required_type(TypeId::of::<T>())
    }

    /// Add a [`TypeId`] to the list of required [`Component`]s.
    ///
    /// Returns `true` if the [`TypeId`] was not already in the list.
    pub fn add_required_type(&mut self, type_id: TypeId) -> bool { self.components.insert(type_id) }

    /// Check if an entity contains all the required [`Component`]s.
    #[inline]
    #[must_use]
    pub fn check(&self, entity: Entity, world: &World) -> bool {
        self.check_ref(world.entity(entity))
    }

    /// Check if an entity contains all the required [`Component`]s.
    #[must_use]
    pub fn check_ref(&self, entity_ref: EntityRef<'_>) -> bool {
        self.components.iter().all(|type_id| entity_ref.contains_type_id(*type_id))
    }
}
