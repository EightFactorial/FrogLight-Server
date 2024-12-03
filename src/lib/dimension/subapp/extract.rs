use bevy::{
    app::{AppLabel, InternedAppLabel},
    prelude::*,
    utils::HashMap,
};
use derive_more::derive::From;

use super::DimensionIdentifier;

pub(super) fn build(app: &mut App) {
    app.init_resource::<DimensionEventQueue>();
    app.add_observer(DimensionMarker::on_add);
    app.add_observer(DimensionMarker::on_remove);
}

/// A marker component for entities that belong to a dimension.
///
/// Adding this component will cause a linked entity
/// to be spawned in the dimension's [`SubApp`].
#[derive(Debug, Deref, Component)]
pub struct DimensionMarker(InternedAppLabel);
impl DimensionMarker {
    /// Create a new [`DimensionMarker`] from an [`AppLabel`].
    #[must_use]
    pub fn new(label: impl AppLabel) -> Self { Self(label.intern()) }
}
impl std::fmt::Display for DimensionMarker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{:?}", self.0) }
}
impl<A: AppLabel> From<A> for DimensionMarker {
    fn from(label: A) -> Self { Self(label.intern()) }
}

#[derive(Debug, Default, Deref, DerefMut, Resource)]
struct DimensionEventQueue {
    queues: HashMap<InternedAppLabel, Vec<DimensionSyncEvent>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DimensionSyncEvent {
    /// The [`App`] [`Entity`] to spawn in the [`SubApp`].
    Add(Entity),
    /// The [`SubApp`] [`Entity`] to despawn.
    Remove(Entity),
}

impl DimensionMarker {
    /// The [`Observer`] for the [`OnAdd`] event.
    ///
    /// Adds a [`DimensionSyncEvent::Add`] event to the [`DimensionEventQueue`].
    fn on_add(
        trigger: Trigger<OnAdd, Self>,
        query: Query<&DimensionMarker>,
        mut queue: ResMut<DimensionEventQueue>,
    ) {
        let marker = query.get(trigger.entity()).unwrap();
        queue.entry(**marker).or_default().push(DimensionSyncEvent::Add(trigger.entity()));
    }

    /// The [`Observer`] for the [`OnRemove`] event.
    ///
    /// Adds a [`DimensionSyncEvent::Remove`] event to the
    /// [`DimensionEventQueue`], or for an in-progress event,
    /// removes the queued [`DimensionSyncEvent::Add`] event.
    fn on_remove(
        trigger: Trigger<OnRemove, Self>,
        query: Query<(&DimensionMarker, Option<&DimensionTracker>)>,
        mut queue: ResMut<DimensionEventQueue>,
        mut commands: Commands,
    ) {
        if let Ok((marker, tracker)) = query.get(trigger.entity()) {
            let marker = **marker;
            if let Some(tracker) = tracker {
                // Queue a despawn event
                queue.entry(marker).or_default().push(DimensionSyncEvent::Remove(**tracker));
                commands.entity(trigger.entity()).remove::<DimensionTracker>();
            } else if let Some(position) = queue.get(&marker).and_then(|queue| queue.iter().position(|event| matches!(event, DimensionSyncEvent::Add(entity) if *entity == trigger.entity()))) {
                // Find and remove the queued spawn event
                queue.get_mut(&marker).unwrap().swap_remove(position);
            } else {
                error!("Failed to remove DimensionMarker: No tracked entity or queued event found");
            }
        } else {
            error!("Failed to remove DimensionMarker: Components not found");
        }
    }
}

/// A component containing the
/// linked entity in a dimension's [`SubApp`].
#[derive(Debug, Deref, Component)]
pub struct DimensionTracker(Entity);

/// A component containing the
/// linked entity in the main [`App`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, From, Component)]
struct MainAppMarker(Entity);

/// Extracts queued events from the [`DimensionEventQueue`]
pub(super) fn extract(app: &mut World, sub_app: &mut World) {
    sub_app.resource_scope::<DimensionIdentifier, _>(|sub_app, ident| {
        app.resource_scope::<DimensionEventQueue, _>(|app, mut event_queue| {
            let Some(queue) = event_queue.get_mut(&**ident) else {
                return;
            };

            for event in queue.drain(..) {
                match event {
                    DimensionSyncEvent::Add(entity) => {
                        let spawned = sub_app.spawn(MainAppMarker(entity));
                        app.entity_mut(entity).insert(DimensionTracker(spawned.id()));

                        #[cfg(debug_assertions)]
                        trace!("App Entity {entity} linked to SubApp Entity {}", spawned.id());
                    }
                    DimensionSyncEvent::Remove(entity) => {
                        sub_app.despawn(entity);

                        #[cfg(debug_assertions)]
                        trace!("Despawning SubApp Entity {entity}");
                    }
                }
            }
        });
    });
}
