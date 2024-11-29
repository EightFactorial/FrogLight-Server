//! TODO

use bevy::prelude::*;

mod cursor;
pub use cursor::*;

/// A [`Plugin`] for entity-related systems and resources.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EntityIds>();
        app.init_resource::<EntityUuids>();
    }
}
