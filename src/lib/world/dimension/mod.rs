use bevy::prelude::*;

mod reflect;
pub use reflect::{DimensionTrait, ReflectDimension};

mod storage;
pub use storage::{DimensionMap, DimensionStorage};

mod types;
pub use types::Overworld;

/// A [`Plugin`] for dimension-related systems and resources.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DimensionPlugin;

impl Plugin for DimensionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Overworld>();

        app.init_resource::<DimensionStorage>();
    }
}
