use std::{sync::LazyLock, time::Instant};

use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct ReadyPlugin;

static STARTUP: LazyLock<Instant> = LazyLock::new(Instant::now);

impl Plugin for ReadyPlugin {
    fn build(&self, app: &mut App) {
        // Set the time when the program starts
        let _ = &*STARTUP;

        // Log how long it took for the game to start up
        app.add_systems(PostStartup, ReadyPlugin::log_ready.run_if(run_once));
    }
}

impl ReadyPlugin {
    fn log_ready() {
        #[cfg(feature = "mimalloc")]
        info_once!("Using MiMalloc as the global allocator!");

        let elapsed = STARTUP.elapsed();
        if elapsed.as_secs_f32() >= 0.1 {
            info_once!("Done ({:.2}s)!", elapsed.as_secs_f32());
        } else if elapsed.as_millis() > 0 {
            info_once!("Done ({}ms)!", elapsed.as_millis());
        } else {
            info_once!("Done ({}µs)!", elapsed.as_micros());
        }
    }
}
