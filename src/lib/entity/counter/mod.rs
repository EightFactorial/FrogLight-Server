//! TODO

use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

use bevy::prelude::*;
use froglight::prelude::EntityId;

use crate::dimension::{All, DimensionApp};

/// A plugin for assigning unique [`EntityId`]s.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityCounterPlugin;

impl Plugin for EntityCounterPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EntityIds>();

        let counter = EntityIds::default();
        app.insert_dimension_resource(All, counter.clone());
        app.insert_resource(counter);
    }
}

/// A [`Resource`] for generating unique [`EntityId`]s.
#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct EntityIds {
    counter: Arc<AtomicU32>,
}

impl EntityIds {
    /// Create a new [`EntityIds`] instance.
    #[must_use]
    pub fn new() -> Self { Self { counter: Arc::new(AtomicU32::new(0)) } }

    /// Create a new [`EntityId`].
    ///
    /// Guaranteed to be unique.
    #[must_use]
    pub fn create(&self) -> EntityId { EntityId::new(self.counter.fetch_add(1, Ordering::Relaxed)) }
}

impl Default for EntityIds {
    fn default() -> Self { Self::new() }
}
