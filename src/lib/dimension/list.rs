//! TODO

use bevy::{
    app::{App, InternedAppLabel},
    prelude::{AppTypeRegistry, Deref, FromWorld, Resource, World},
};

use super::{All, DimensionApp, ReflectDimension};

#[doc(hidden)]
pub(super) fn finish(app: &mut App) {
    let list = DimensionList::from_world(app.world_mut());
    app.insert_dimension_resource(All, list.clone());
    app.insert_resource(list);
}

/// A list of all dimensions.
#[derive(Debug, Clone, PartialEq, Deref, Resource)]
pub struct DimensionList {
    #[deref]
    list: Vec<ReflectDimension>,
    labels: Vec<InternedAppLabel>,
}

impl DimensionList {
    /// Get a [`ReflectDimension`] by its [`InternedAppLabel`].
    #[must_use]
    pub fn get(&self, label: InternedAppLabel) -> Option<&ReflectDimension> {
        self.index_of(label).and_then(|i| self.list.get(i))
    }

    /// Get the index of a [`ReflectDimension`] by its [`InternedAppLabel`].
    #[must_use]
    pub fn index_of(&self, label: InternedAppLabel) -> Option<usize> {
        self.labels.iter().position(|l| *l == label)
    }
}

impl FromWorld for DimensionList {
    fn from_world(world: &mut World) -> Self {
        let registry = world.resource::<AppTypeRegistry>().read();

        let mut list: Vec<_> =
            registry.iter_with_data::<ReflectDimension>().map(|(_, d)| d.clone()).collect();
        list.sort_by(|a, b| a.dimension_id.cmp(&b.dimension_id));

        let labels: Vec<_> = list
            .iter()
            .map(|d| d.app_label.expect("Registered Dimension has no AppLabel!"))
            .collect();

        Self { list, labels }
    }
}
