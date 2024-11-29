use bevy::{prelude::Resource, utils::HashSet};
use froglight::prelude::{EntityUuid, Uuid};

/// A [`Resource`] that provides unique [`EntityUuid`]s.
#[derive(Debug, Default, Resource)]
pub struct EntityUuids {
    #[expect(dead_code)]
    hashset: HashSet<Uuid>,
}

impl EntityUuids {
    /// Get a random [`EntityUuid`].
    #[must_use]
    pub fn get(&self) -> EntityUuid { todo!() }
}
