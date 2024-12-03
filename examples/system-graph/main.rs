//! Prints graphs of bevy apps and subapps.
//!
//! Useful for visualizing schedule and system ordering

use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

use bevy::{
    app::{AppLabel, InternedAppLabel, MainScheduleOrder},
    ecs::schedule::{InternedScheduleLabel, ScheduleLabel},
    prelude::*,
};
use bevy_mod_debugdump::schedule_graph::settings::{
    Settings as ScheduleSettings, Style as ScheduleStyle,
};
use froglight::prelude::*;
use froglight_server::{
    dimension::{Nether, Overworld},
    ServerPlugins,
};

static SUBAPPS: LazyLock<[InternedAppLabel; 2]> =
    LazyLock::new(|| [Overworld.intern(), Nether.intern()]);

/// Bevy's fixed [`bevy::app::main_schedule`].
///
/// Run every fixed period of time.
static BEVY_FIXED_SCHEDULES: LazyLock<[InternedScheduleLabel; 5]> = LazyLock::new(|| {
    [
        FixedFirst.intern(),
        FixedPreUpdate.intern(),
        FixedUpdate.intern(),
        FixedPostUpdate.intern(),
        FixedLast.intern(),
    ]
});

/// Schedules defined in the [`froglight-utils`] crate.
///
/// Run every fixed period of time.
static UTIL_FIXED_SCHEDULES: LazyLock<[InternedScheduleLabel; 8]> = LazyLock::new(|| {
    [
        OneTick.intern(),
        TwoTicks.intern(),
        TenTicks.intern(),
        QuarterSecond.intern(),
        HalfSecond.intern(),
        OneSecond.intern(),
        TwoSeconds.intern(),
        FiveSeconds.intern(),
    ]
});

fn main() {
    let mut app = App::new();
    app.add_plugins(ServerPlugins::default());
    app.finish();

    graph_world(app.world_mut(), "Main");
    for label in SUBAPPS.iter() {
        let subapp = app.sub_app_mut(*label);
        graph_world(subapp.world_mut(), &format!("{label:?}"));
    }
}

fn graph_world(world: &mut World, app: &str) {
    // Generate schedule graphs
    graph_schedules(world, app, "main", &[Main.intern()]);

    let startup_labels = world.resource::<MainScheduleOrder>().startup_labels.clone();
    graph_schedules(world, app, "startup", &startup_labels);

    let labels = world.resource::<MainScheduleOrder>().labels.clone();
    graph_schedules(world, app, "update", &labels);

    graph_schedules(world, app, "bevy_fixed", &*BEVY_FIXED_SCHEDULES);
    graph_schedules(world, app, "froglight_fixed", &*UTIL_FIXED_SCHEDULES);
}

/// Generate graphs for the given schedules.
fn graph_schedules(
    world: &mut World,
    app: &str,
    folder: &str,
    schedules: &[InternedScheduleLabel],
) {
    let settings = ScheduleSettings { style: ScheduleStyle::dark_github(), ..Default::default() };

    for label in schedules {
        // Skip schedules that don't exist
        if !world.resource::<Schedules>().contains(*label) {
            warn!("Skipping {app} graph for `{label:?}`, schedule does not exist");
            continue;
        }

        // Generate the graph
        info!("Generating {app} graph for `{label:?}`");
        let graph = world.resource_scope::<Schedules, _>(|world, mut schedules| {
            let ignored_ambiguities = schedules.ignored_scheduling_ambiguities.clone();

            let schedule = schedules.get_mut(*label).unwrap();
            schedule.graph_mut().initialize(world);
            let _ = schedule.graph_mut().build_schedule(
                world.components(),
                ScheduleDebugGroup.intern(),
                &ignored_ambiguities,
            );

            bevy_mod_debugdump::schedule_graph::schedule_graph_dot(schedule, world, &settings)
        });

        // Write the graph to a file
        write_dot_and_convert(graph, &format!("{label:?}"), &graph_path(app, folder));
    }
}

/// Write the graph to a dot file and convert it to an svg.
fn write_dot_and_convert(graph: String, label: &str, path: &Path) {
    // Get the path to write the graph to
    let path = path.join(format!("{label}.dot"));
    debug!("Writing `{label}` to \"{}\"", truncate_path(&path));

    // Write the graph to a file
    if let Err(err) = std::fs::write(&path, graph) {
        error!("Failed to write `{label}`: {err}");
    }

    // Convert the graph to an image
    let output_path = path.with_extension("svg");
    debug!("Converting \"{}\" to \"{}\"", truncate_path(&path), truncate_path(&output_path));

    if let Err(err) = std::process::Command::new("dot")
        .arg("-Tsvg")
        .arg(&path)
        .arg("-o")
        .arg(&output_path)
        .output()
    {
        error!("Failed to convert \"{}\" to \"{}\": {err}", path.display(), output_path.display());
    }
}

/// Get the path to write the graphs to.
fn graph_path(subapp: &str, folder: &str) -> PathBuf {
    // Get the path to write the graphs to
    let mut path = PathBuf::from(file!());

    path.pop();
    path.push("graphs");
    path.push(subapp);
    path.push(folder);

    if !path.exists() {
        std::fs::create_dir_all(&path).expect("Failed to create directory");
    }

    path
}

/// Truncate the path to just the file name.
fn truncate_path(path: &Path) -> &str {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_else(|| path.to_str().unwrap_or("unknown"))
}

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
struct ScheduleDebugGroup;
