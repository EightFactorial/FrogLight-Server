//! TODO

use bevy::{
    app::{InternedAppLabel, MainScheduleOrder, MainSchedulePlugin},
    ecs::{event::EventRegistry, schedule::ScheduleLabel},
    prelude::*,
};
use derive_more::derive::From;

mod component;
pub use component::SubAppComponents;

mod extract;
pub use extract::{MainAppEvent, SubAppEvent, SubAppEventQueue};

mod schedule;
pub use schedule::Network;

mod traits;
pub use traits::{All, DimensionApp, DimensionType};

use super::ReflectDimension;

mod marker;
pub use marker::{DimensionMarker, MainAppMarker, SubAppTracker};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    build_dimension_subapps(app);

    extract::build(app);
    marker::build(app);
}

/// An identifier inserted into the world for each dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, From, Resource)]
pub struct DimensionIdentifier {
    dimension: InternedAppLabel,
}
impl DimensionIdentifier {
    /// Returns the [`InternedAppLabel`] for the dimension.
    #[must_use]
    pub const fn label(&self) -> InternedAppLabel { self.dimension }
}
impl<T: DimensionType> From<T> for DimensionIdentifier {
    fn from(_: T) -> Self { Self::from(T::default().intern()) }
}

/// For each registered [`Dimension`](ReflectDimension),
/// build a [`SubApp`] if one does not already exist.
fn build_dimension_subapps(app: &mut App) {
    let dimensions: Vec<_> = app
        .world()
        .resource::<AppTypeRegistry>()
        .read()
        .iter_with_data::<ReflectDimension>()
        .filter_map(|(t, d)| d.app_label.as_ref().map(|label| (t.clone(), *label)))
        .collect();

    for (dimension, label) in dimensions {
        let dimension = dimension.type_info().type_path_table().short_path();

        if app.get_sub_app(label).is_none() {
            debug!("Building SubApp for dimension \"{dimension}\"");
            let mut sub_app = build_subapp(app);
            sub_app.insert_resource(DimensionIdentifier::from(label));

            app.insert_sub_app(label, sub_app);
        } else {
            debug!("Skipping SubApp for dimension \"{dimension}\", already exists");
        }
    }
}

/// Build a [`SubApp`] that runs in parallel with the main app.
fn build_subapp(app: &mut App) -> SubApp {
    let mut sub_app = SubApp::new();

    // Set the extract function
    sub_app.set_extract(extract::extract);

    // Copy the `AppTypeRegistry`
    let registry = app.world().resource::<AppTypeRegistry>().clone();
    sub_app.world_mut().insert_resource(registry);

    // Initialize the `EventRegistry`
    sub_app.init_resource::<EventRegistry>();
    sub_app.add_systems(
        First,
        bevy::ecs::event::event_update_system
            .in_set(bevy::ecs::event::EventUpdates)
            .run_if(bevy::ecs::event::event_update_condition),
    );

    // Add the `MainSchedulePlugin`
    sub_app.add_plugins(MainSchedulePlugin);
    sub_app.update_schedule = Some(Main.intern());

    // Insert `Network` after `First` and clear startup schedules
    let mut schedule_order = sub_app.world_mut().resource_mut::<MainScheduleOrder>();
    schedule_order.insert_after(First, Network);
    schedule_order.startup_labels = Vec::new();

    // Initialize all schedules
    for schedule in schedule_order.labels.clone() {
        sub_app.init_schedule(schedule);
    }

    sub_app
}
