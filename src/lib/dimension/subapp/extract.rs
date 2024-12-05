use bevy::{prelude::*, utils::HashMap};
use derive_more::derive::Deref;

use super::{
    marker::SubAppTracker, All, DimensionApp, DimensionIdentifier, MainAppMarker, SubAppComponents,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.add_dimension_event::<MainAppEvent>(All);
    app.init_resource::<SubAppEventQueue>();
}

/// An [`Event`] to execute in the main [`App`].
#[derive(Debug, Event)]
pub enum MainAppEvent {
    /// Insert a [`Component`] into a main [`App`] entity.
    InsertComponent(MainAppMarker, Box<dyn PartialReflect>),
}

/// A queue of [`SubAppEvent`]s to execute,
/// indexed by [`DimensionIdentifier`].
#[derive(Debug, Default, Deref, DerefMut, Resource)]
pub struct SubAppEventQueue {
    queue: HashMap<DimensionIdentifier, Vec<SubAppEvent>>,
}

/// An [`Event`] to execute in a [`SubApp`].
#[derive(Debug)]
pub enum SubAppEvent {
    /// Spawn a linked entity in the [`SubApp`].
    SpawnLinked(MainAppMarker),
    /// Despawn a linked entity in the [`SubApp`].
    DespawnLinked(SubAppTracker),
}

pub(super) fn extract(app: &mut World, sub_app: &mut World) {
    // Execute all `SubAppEvents` for this SubApp's `DimensionIdentifier`
    app.resource_scope::<SubAppEventQueue, _>(|app, mut queue| {
        let identifier = *sub_app.resource::<DimensionIdentifier>();
        for event in queue.entry(identifier).or_default().drain(..) {
            match event {
                SubAppEvent::SpawnLinked(marker) => {
                    // Spawn an Entity in the SubApp.
                    let mut entity = sub_app.spawn(marker);

                    // Take any `SubAppComponents` from the Entity in the main App.
                    if let Some(components) = app.entity_mut(*marker).take::<SubAppComponents>() {
                        components.write_to(&mut entity);
                    }

                    // Add a `SubAppTracker` to the Entity in the main App.
                    app.entity_mut(*marker).insert(SubAppTracker(entity.id()));
                }
                SubAppEvent::DespawnLinked(tracker) => {
                    // If the Entity exists in the main App, collect its `SubAppComponents`.
                    if let Some(mut entity) = sub_app
                        .get::<MainAppMarker>(*tracker)
                        .and_then(|marker| app.get_entity_mut(**marker).ok())
                    {
                        sub_app.entity_mut(*tracker).remove::<MainAppMarker>();
                        entity.insert(SubAppComponents::read_from(*tracker, sub_app));
                    }

                    // Despawn the Entity in the SubApp.
                    sub_app.entity_mut(*tracker).despawn_recursive();
                }
            }
        }
    });

    // Execute all `MainAppEvents` from the SubApp.
    sub_app.resource_scope::<Events<MainAppEvent>, _>(|sub_app, mut events| {
        for event in events.drain() {
            match event {
                MainAppEvent::InsertComponent(marker, component) => {
                    if let Ok(mut entity) = app.get_entity_mut(*marker) {
                        // Get the `AppTypeRegistry`.
                        let registry = sub_app.resource::<AppTypeRegistry>().read();

                        // Get the `ReflectComponent` for the component.
                        if let Some(reflect) =
                            component.get_represented_type_info().and_then(|info| {
                                registry.get_type_data::<ReflectComponent>(info.type_id())
                            })
                        {
                            // Apply or insert the component into the entity.
                            reflect.apply_or_insert(
                                &mut entity,
                                component.as_partial_reflect(),
                                &registry,
                            );
                        } else {
                            warn!(
                                "Failed to get ReflectComponent for type: {}",
                                component.reflect_short_type_path()
                            );
                        }
                    }
                }
            }
        }
    });
}
