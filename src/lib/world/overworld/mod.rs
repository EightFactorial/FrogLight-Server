//! TODO

use bevy::prelude::*;

/// A plugin for the overworld.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, _app: &mut App) {}
}
