use std::any::TypeId;

use bevy::{
    prelude::*,
    utils::{Entry, HashMap, TypeIdMap},
};
use froglight::{prelude::ChunkPosition, world::Chunk};

use super::DimensionTrait;

/// A set of [`DimensionMap`]s indexed by dimension type.
#[derive(Debug, Default, Resource)]
pub struct DimensionStorage(TypeIdMap<DimensionMap>);

/// A dimension made from [`Chunk`]s.
#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct DimensionMap(HashMap<ChunkPosition, Chunk>);

impl DimensionStorage {
    /// Get a [`DimensionMap`] by dimension type.
    #[must_use]
    pub fn get_dimension<D: DimensionTrait>(&self) -> Option<&DimensionMap> {
        self.0.get(&TypeId::of::<D>())
    }

    /// Get a mutable [`DimensionMap`] by dimension type.
    #[must_use]
    pub fn get_dimension_mut<D: DimensionTrait>(&mut self) -> Option<&mut DimensionMap> {
        self.0.get_mut(&TypeId::of::<D>())
    }

    /// Get a [`DimensionMap`] by dimension type,
    /// or create a new one if it doesn't exist.
    #[must_use]
    pub fn get_dimension_mut_or_default<D: DimensionTrait>(&mut self) -> &mut DimensionMap {
        self.0.entry(TypeId::of::<D>()).or_default()
    }

    /// Insert a [`DimensionMap`] by dimension type.
    ///
    /// Returns the previous [`DimensionMap`], if one existed.
    pub fn insert_dimension<D: DimensionTrait>(
        &mut self,
        world: DimensionMap,
    ) -> Option<DimensionMap> {
        self.0.insert(TypeId::of::<D>(), world)
    }
}

impl DimensionStorage {
    /// Get a [`Chunk`] by dimension type and position.
    #[must_use]
    pub fn get_chunk<D: DimensionTrait>(&self, position: &ChunkPosition) -> Option<&Chunk> {
        self.get_dimension::<D>()?.get(position)
    }

    /// Get a mutable [`Chunk`] by dimension type and position.
    #[must_use]
    pub fn get_chunk_mut<D: DimensionTrait>(
        &mut self,
        position: &ChunkPosition,
    ) -> Option<&mut Chunk> {
        self.get_dimension_mut::<D>()?.get_mut(position)
    }

    /// Get a [`Chunk`]'s [`Entry`] by dimension type and position.
    #[must_use]
    pub fn chunk_entry<D: DimensionTrait>(
        &mut self,
        position: ChunkPosition,
    ) -> Entry<'_, ChunkPosition, Chunk> {
        self.get_dimension_mut_or_default::<D>().entry(position)
    }

    /// Insert a [`Chunk`] by dimension type and position.
    ///
    /// Returns the previous [`Chunk`], if one existed.
    pub fn insert_chunk<D: DimensionTrait>(
        &mut self,
        position: ChunkPosition,
        chunk: Chunk,
    ) -> Option<Chunk> {
        self.get_dimension_mut_or_default::<D>().insert(position, chunk)
    }
}
