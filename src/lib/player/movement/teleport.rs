use bevy::{prelude::*, utils::HashMap};

/// A [`Component`] that manages teleport requests.
#[derive(Debug, Default, PartialEq, Component, Reflect)]
#[reflect(Default, Component)]
pub struct TeleportCounter {
    current: u32,
    pending: HashMap<u32, Transform>,
}

impl TeleportCounter {
    /// Create a new teleport request, returning the ID.
    #[must_use]
    pub fn create_new(&mut self, transform: Transform) -> u32 {
        let id = self.current;
        self.current += 1;
        self.pending.insert(id, transform);
        id
    }

    /// Accept a teleport request.
    #[must_use]
    pub fn accept(&mut self, teleport_id: u32) -> Option<Transform> {
        // Deny any non-existent teleport requests.
        if teleport_id > self.current {
            return None;
        }
        // Remove all older teleport requests.
        self.pending.retain(|id, _| id > &teleport_id);
        // Remove the teleport request.
        self.pending.remove(&teleport_id)
    }

    /// Get the position of a pending teleport request.
    #[must_use]
    pub fn get(&self, teleport_id: &u32) -> Option<&Transform> { self.pending.get(teleport_id) }

    /// Return the number of pending teleport requests.
    #[must_use]
    pub fn pending(&self) -> usize { self.pending.len() }

    /// Check if there are any pending teleport requests.
    #[must_use]
    pub fn any_pending(&self) -> bool { !self.pending.is_empty() }
}
