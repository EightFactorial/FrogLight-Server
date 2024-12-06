use bevy::{prelude::*, utils::HashMap};
use derive_more::derive::Deref;

use super::{
    marker::SubAppTracker, All, DimensionApp, DimensionIdentifier, DimensionMarker, MainAppMarker,
    SubAppComponents,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<SubAppEventQueue>();
    app.init_resource::<SubAppTransferQueue>();

    app.add_dimension_event::<MainAppEvent>(All);
}

/// An [`Event`] to execute in the main [`App`].
#[derive(Debug, Event)]
pub enum MainAppEvent {
    /// Change the associated [`SubApp`] of a main [`App`] entity.
    ///
    /// Must only be used on linked entities with a [`MainAppMarker`].
    ChangeWorld(MainAppMarker, DimensionIdentifier),
    /// Transfer an entity from one [`SubApp`] to another.
    ///
    /// Must only be used on non-linked entities.
    TransferEntity(Entity, DimensionIdentifier),
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
    /// Insert a [`Component`] into a [`SubApp`] entity.
    InsertComponent(SubAppTracker, Box<dyn PartialReflect>),
}

#[allow(clippy::too_many_lines)]
pub(super) fn extract(app: &mut World, sub_app: &mut World) {
    // Process events from the SubApp.
    sub_app.resource_scope::<Events<MainAppEvent>, _>(|sub_app, mut events| {
        // Execute all `MainAppEvents` from the SubApp.
        for event in events.drain() {
            match event {
                MainAppEvent::ChangeWorld(marker, identifier) => {
                    if let Ok(mut entity) = app.get_entity_mut(*marker) {
                        entity
                            .remove::<DimensionMarker>()
                            .insert(DimensionMarker::from(identifier));
                    } else {
                        warn!("Failed to change world: Entity not found");
                    }
                }
                MainAppEvent::TransferEntity(entity, identifier) => {
                    if let Ok(entity) = sub_app.get_entity_mut(entity) {
                        app.resource_mut::<SubAppTransferQueue>()
                            .entry(identifier)
                            .or_default()
                            .push(SubAppComponents::read_from(entity.id(), sub_app));
                    } else {
                        warn!("Failed to transfer entity: Entity not found");
                    }
                }
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
                    } else {
                        warn!(
                            "Failed to insert \"{}\": Entity not found",
                            component.reflect_short_type_path()
                        );
                    }
                }
            }
        }
    });

    // Process events from the main App.
    app.resource_scope::<SubAppEventQueue, _>(|app, mut queue| {
        let identifier = *sub_app.resource::<DimensionIdentifier>();

        // Execute all `SubAppEvents` for this SubApp's `DimensionIdentifier`
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
                SubAppEvent::InsertComponent(tracker, component) => {
                    if let Ok(mut entity) = sub_app.get_entity_mut(*tracker) {
                        // Get the `AppTypeRegistry`.
                        let registry = app.resource::<AppTypeRegistry>().read();

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

        // Process the `SubAppTransferQueue` for this SubApp's `DimensionIdentifier`.
        for components in
            app.resource_mut::<SubAppTransferQueue>().entry(identifier).or_default().drain(..)
        {
            components.write_to(&mut sub_app.spawn_empty());
        }
    });
}

/// A queue of [`SubAppComponents`] to transfer across [`SubApp`]s,
#[derive(Debug, Default, Deref, DerefMut, Resource)]
struct SubAppTransferQueue {
    queue: HashMap<DimensionIdentifier, Vec<SubAppComponents>>,
}
