//! TODO

use bevy::prelude::*;

pub mod plugin;
use plugin::ServerPlugins;

fn main() -> AppExit { App::new().add_plugins(ServerPlugins::default()).run() }
