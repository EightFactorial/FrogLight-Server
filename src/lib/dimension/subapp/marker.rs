use bevy::prelude::*;
use derive_more::derive::From;

use super::{
    extract::{SubAppEvent, SubAppEventQueue},
    DimensionIdentifier, DimensionType,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.add_observer(DimensionMarker::on_add);
    app.add_observer(DimensionMarker::on_remove);
}

/// A marker component for entities that belong to a dimension.
///
/// Adding this component will cause a linked entity
/// to be spawned in the dimension's [`SubApp`].
#[derive(Debug, Clone, Copy, Deref, Component)]
pub struct DimensionMarker(pub DimensionIdentifier);

/// A component containing the linked entity in a [`SubApp`].
///
/// Must only be added to entities in the main [`App`].
#[derive(Debug, Clone, Copy, Deref, Component)]
pub struct SubAppTracker(pub Entity);

/// A component containing the
/// linked entity in the main [`App`].
///
/// Must only be added to entities in a [`SubApp`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, From, Component)]
pub struct MainAppMarker(pub Entity);

impl DimensionMarker {
    /// Create a new [`DimensionMarker`] from
    /// any type that implements [`DimensionType`].
    #[must_use]
    pub fn new(label: impl DimensionType) -> Self { Self(DimensionIdentifier::from(label)) }

    /// The [`Observer`] for the [`OnAdd`] event.
    ///
    /// Adds a [`SubAppEvent::SpawnLinked`] event to the [`SubAppEventQueue`].
    fn on_add(
        trigger: Trigger<OnAdd, Self>,
        mut query: Query<&DimensionMarker>,
        mut queue: ResMut<SubAppEventQueue>,
    ) {
        let marker = query.get_mut(trigger.entity()).unwrap();
        let queue = queue.entry(**marker).or_default();
        queue.push(SubAppEvent::SpawnLinked(MainAppMarker(trigger.entity())));
    }

    /// The [`Observer`] for the [`OnRemove`] event.
    ///
    /// Adds a [`SubAppEvent::DespawnLinked`] event to the
    /// [`SubAppEventQueue`], or for an in-progress event,
    /// removes the queued [`SubAppEvent::SpawnLinked`] event.
    fn on_remove(
        trigger: Trigger<OnRemove, Self>,
        query: Query<(&DimensionMarker, Option<&SubAppTracker>)>,
        mut queue: ResMut<SubAppEventQueue>,
        mut commands: Commands,
    ) {
        if let Ok((marker, tracker)) = query.get(trigger.entity()) {
            let queue = queue.entry(**marker).or_default();
            if let Some(tracker) = tracker {
                // Queue a despawn event
                queue.push(SubAppEvent::DespawnLinked(*tracker));
                commands.entity(trigger.entity()).remove::<SubAppTracker>();
            } else if let Some(position) = queue.iter().position(|event| matches!(event, SubAppEvent::SpawnLinked(entity) if **entity == trigger.entity())) {
                // Find and remove the queued spawn event
                queue.swap_remove(position);
            } else {
                error!("Failed to remove DimensionMarker: No tracked entity or queued event found");
            }
        } else {
            error!("Failed to remove DimensionMarker: Components not found");
        }
    }
}

impl std::fmt::Display for DimensionMarker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{:?}", self.0) }
}

impl<D: DimensionType> From<D> for DimensionMarker {
    fn from(label: D) -> Self { Self::new(label) }
}
impl From<DimensionIdentifier> for DimensionMarker {
    fn from(ident: DimensionIdentifier) -> Self { Self(ident) }
}
