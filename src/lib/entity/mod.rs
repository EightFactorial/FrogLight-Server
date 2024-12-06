//! TODO

use bevy::{app::PluginGroupBuilder, prelude::*};

mod counter;
pub use counter::{EntityCounterPlugin, EntityIds};

/// A [`PluginGroup`] that adds entity-related plugins to the [`App`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityPlugins;

impl PluginGroup for EntityPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(EntityCounterPlugin)
    }
}
