use bevy::prelude::{AppTypeRegistry, FromWorld, Resource, World};

use super::ReflectDimension;

/// A list of all registered dimensions.
#[derive(Debug, Clone, PartialEq, Resource)]
pub struct DimensionList {
    dimensions: Vec<ReflectDimension>,
    /// The index of the `Overworld` dimension in the list.
    index_offset: isize,
}

impl FromWorld for DimensionList {
    fn from_world(world: &mut World) -> Self {
        let mut list = DimensionList::new_empty();
        list.refresh(world);
        list
    }
}

impl DimensionList {
    /// Create a new empty [`DimensionList`].
    #[must_use]
    pub const fn new_empty() -> Self { DimensionList { dimensions: Vec::new(), index_offset: 0 } }

    /// Iterate over all registered dimensions.
    ///
    /// # Warning
    /// Do **NOT** use `enumerate` for the index of the dimension!
    ///
    /// If you need the indexes, use [`DimensionList::iter_with_index`] instead.
    pub fn iter(&self) -> impl Iterator<Item = &ReflectDimension> { self.dimensions.iter() }

    /// Iterate over all registered dimensions with their index.
    #[expect(clippy::cast_possible_wrap)]
    pub fn iter_with_index(&self) -> impl Iterator<Item = (isize, &ReflectDimension)> {
        self.dimensions.iter().enumerate().map(|(i, d)| (i as isize + self.index_offset, d))
    }

    /// Get the index of a dimension by its key.
    #[must_use]
    #[expect(clippy::cast_possible_wrap)]
    pub fn index_of(&self, dimension: &str) -> Option<isize> {
        self.dimensions
            .iter()
            .position(|data| data.dimension_key == dimension)
            .map(|i| i as isize + self.index_offset)
    }

    /// Update the [`DimensionList`] with all registered dimensions.
    #[expect(clippy::cast_possible_wrap)]
    pub fn refresh(&mut self, world: &World) {
        let registry = world.resource::<AppTypeRegistry>();
        let registry = registry.read();

        // Collect and sort all registered dimensions.
        let mut registered = Vec::new();
        for (_, dimension_data) in registry.iter_with_data::<ReflectDimension>() {
            registered.push(dimension_data);
        }
        registered.sort_by_key(|data| &data.dimension_key);

        // Update the `DimensionList`.
        self.index_offset = registered
            .iter()
            .position(|data| data.dimension_key == "minecraft:overworld")
            .unwrap_or_default() as isize;
        self.dimensions = registered.into_iter().cloned().collect();
    }
}
