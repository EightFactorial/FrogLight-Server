//! TODO

use std::sync::atomic::{AtomicU32, Ordering};

use bevy::prelude::*;
use froglight::prelude::EntityId;

use crate::dimension::{All, DimensionApp};

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.init_dimension_resource::<EntityIds>(All); }

/// A counter guaranteed to produce unique [`EntityId`]s.
#[derive(Debug, Default, Resource)]
pub struct EntityIds {
    counter: AtomicU32,
}

impl EntityIds {
    /// Create a new, unique [`EntityId`]
    #[must_use]
    pub fn next(&self) -> EntityId { EntityId(self.counter.fetch_add(1, Ordering::Relaxed)) }
}
