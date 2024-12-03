use bevy::ecs::schedule::ScheduleLabel;

/// A [`Schedule`](bevy::ecs::schedule::Schedule) for network-related systems.
///
/// Runs after [`First`](bevy::app::First), but before
/// [`PreUpdate`](bevy::app::PreUpdate).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct Network;
