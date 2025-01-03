//! TODO

use bevy::prelude::*;

mod list;
pub use list::DimensionList;

mod reflect;
pub use reflect::{DimensionTrait, MonsterSpawnLightLevel, ReflectDimension};

pub mod subapp;
pub use subapp::{All, DimensionApp, Network};

mod types;
pub use types::{Nether, Overworld};

/// TODO
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DimensionPlugin;

impl Plugin for DimensionPlugin {
    fn build(&self, app: &mut App) {
        types::build(app);

        subapp::build(app);
    }

    fn finish(&self, app: &mut App) { list::finish(app); }
}
