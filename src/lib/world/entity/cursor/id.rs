use std::sync::atomic::{AtomicU32, Ordering};

use bevy::prelude::Resource;
use froglight::prelude::EntityId;

/// A [`Resource`] that provides unique [`EntityId`]s.
#[derive(Debug, Default, Resource)]
pub struct EntityIds {
    index: AtomicU32,
}

impl EntityIds {
    /// Get the next [`EntityId`].
    #[must_use]
    pub fn get(&self) -> EntityId { EntityId(self.index.fetch_add(1, Ordering::SeqCst)) }
}
