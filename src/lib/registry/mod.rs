//! TODO

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod dimension;
use dimension::DimensionPlugin;

/// A [`PluginGroup`] for all registry plugins.
///
/// Contains:
/// - [`DimensionPlugin`]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegistryPlugins;

impl PluginGroup for RegistryPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(DimensionPlugin)
    }
}
