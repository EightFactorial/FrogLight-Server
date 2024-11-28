use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::prelude::Version;

use super::ServerBrand;

type ChecklistFn = dyn Fn(Entity, &World) -> ConfigAction + Send + Sync;
type BoxedCheck = Box<ChecklistFn>;

/// A checklist of functions to run when a player logs in.
#[derive(Resource)]
pub struct ConfigChecklist<V: Version> {
    checklist: Vec<BoxedCheck>,
    _phantom: PhantomData<V>,
}

impl<V: Version> Default for ConfigChecklist<V> {
    fn default() -> Self { Self::new() }
}

impl<V: Version> ConfigChecklist<V> {
    /// Create a new empty [`ConfigChecklist`].
    #[must_use]
    pub const fn new_empty() -> Self { Self { checklist: Vec::new(), _phantom: PhantomData } }

    /// Create a new [`ConfigChecklist`] with the default checks.
    #[must_use]
    pub fn new() -> Self {
        let mut checklist = Self::new_empty();
        checklist.add(ServerBrand::has_sent_brand);
        checklist
    }

    /// Add a function to the checklist.
    pub fn add(&mut self, f: impl Fn(Entity, &World) -> ConfigAction + Send + Sync + 'static) {
        self.checklist.push(Box::new(f));
    }

    /// Add a function to the checklist.
    pub fn add_boxed(&mut self, f: BoxedCheck) { self.checklist.push(f); }

    /// Check if an [`Entity`] has a valid configuration.
    #[must_use]
    pub fn check(&self, entity: Entity, world: &World) -> ConfigAction {
        self.checklist
            .iter()
            .map(|f| f(entity, world))
            .find(|action| matches!(action, ConfigAction::Deny(_)))
            .unwrap_or(ConfigAction::Accept)
    }
}

/// An action to take with a client.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConfigAction {
    /// Accept the client configuration.
    Accept,
    /// Deny the configuration with an optional reason.
    Deny(Option<String>),
}
