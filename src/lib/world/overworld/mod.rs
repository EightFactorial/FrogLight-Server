//! TODO

use bevy::{prelude::*, time::TimePlugin};
use froglight::utils::UtilityPlugin;

use crate::dimension::{DimensionApp, Overworld};

/// A plugin for the overworld.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) { app.in_dimension(Overworld, Self::sub_build); }

    fn finish(&self, app: &mut App) { app.in_dimension(Overworld, Self::sub_finish); }
}

impl OverworldPlugin {
    /// Build the [`Overworld`] dimension.
    fn sub_build(app: &mut SubApp) {
        // Add required bevy plugins
        app.add_plugins(TimePlugin);

        // Add utility tools and schedules
        app.add_plugins(UtilityPlugin);

        // TODO: Spawn Chunk entities

        // TODO: Load/Unload Chunks based on player position
    }

    /// Finish the [`Overworld`] dimension.
    fn sub_finish(_app: &mut SubApp) {}
}
