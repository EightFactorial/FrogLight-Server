//! TODO

use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod overworld;
use overworld::OverworldPlugin;

/// A [`PluginGroup`] that adds world-related plugins to the [`App`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldPlugins;

impl PluginGroup for WorldPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(OverworldPlugin)
    }
}
