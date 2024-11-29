use bevy::prelude::*;

mod list;
pub use list::DimensionList;

mod reflect;
pub use reflect::{DimensionTrait, ReflectDimension};

mod types;
pub use types::Overworld;

/// A [`Plugin`] for dimension-related systems and resources.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DimensionPlugin;

impl Plugin for DimensionPlugin {
    fn build(&self, app: &mut App) { app.register_type::<Overworld>(); }

    fn finish(&self, app: &mut App) { app.init_resource::<DimensionList>(); }
}
