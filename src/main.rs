//! TODO

use bevy::prelude::*;
use froglight_server::ServerPlugins;

#[cfg(feature = "mimalloc")]
#[cfg_attr(feature = "mimalloc", global_allocator)]
static GLOBAL: froglight_server::MiMalloc = froglight_server::MiMalloc;

fn main() -> AppExit { App::new().add_plugins(ServerPlugins::localhost()).run() }
