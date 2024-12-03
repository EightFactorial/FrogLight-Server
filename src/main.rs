//! TODO

use bevy::prelude::*;
use froglight_server::ServerPlugins;

fn main() -> AppExit { App::new().add_plugins(ServerPlugins::localhost()).run() }
