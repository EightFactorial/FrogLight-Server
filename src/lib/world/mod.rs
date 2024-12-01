//! TODO

use bevy::app::{PluginGroup, PluginGroupBuilder};

mod entity;
pub use entity::*;

/// A [`PluginGroup`] for all world plugins.
///
/// Contains:
/// - [`EntityPlugin`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldPlugins;

impl PluginGroup for WorldPlugins {
    fn build(self) -> PluginGroupBuilder { PluginGroupBuilder::start::<Self>().add(EntityPlugin) }
}
