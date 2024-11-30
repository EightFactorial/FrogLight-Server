use bevy::prelude::{AppTypeRegistry, Deref, FromWorld, Resource, World};

use super::ReflectDimension;

/// A list of all registered dimensions.
#[derive(Debug, Clone, PartialEq, Deref, Resource)]
pub struct DimensionList {
    dimensions: Vec<ReflectDimension>,
}

impl FromWorld for DimensionList {
    fn from_world(world: &mut World) -> Self {
        let mut list = DimensionList::new_empty();
        list.refresh(world.resource::<AppTypeRegistry>());
        list
    }
}

impl DimensionList {
    /// Create a new empty [`DimensionList`].
    #[inline]
    #[must_use]
    pub const fn new_empty() -> Self { Self { dimensions: Vec::new() } }

    /// Get a dimension by its key.
    #[must_use]
    #[expect(clippy::cast_possible_truncation)]
    pub fn get_dimension(&self, dimension: &str) -> Option<(&ReflectDimension, u32)> {
        self.dimensions.iter().enumerate().find_map(|(i, data)| {
            if data.dimension_key == dimension {
                Some((data, i as u32))
            } else {
                None
            }
        })
    }

    /// Get the index of a dimension by its key.
    #[must_use]
    #[expect(clippy::cast_possible_truncation)]
    pub fn index_of(&self, dimension: &str) -> Option<u32> {
        self.dimensions.iter().position(|data| data.dimension_key == dimension).map(|i| i as u32)
    }
}

impl DimensionList {
    /// Update the [`DimensionList`] with all registered dimensions.
    pub fn refresh(&mut self, registry: &AppTypeRegistry) {
        let registry = registry.read();

        // Collect and sort all registered dimensions.
        let mut registered = Vec::new();
        for (_, dimension_data) in registry.iter_with_data::<ReflectDimension>() {
            registered.push(dimension_data);
        }
        registered.sort_by_key(|data| &data.dimension_key);

        // Update the `DimensionList`.
        self.dimensions = registered.into_iter().cloned().collect();
    }
}
