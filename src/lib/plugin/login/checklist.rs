use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::prelude::Version;

use super::LoginProfileAction;

type ChecklistFn = dyn Fn(Entity, &World) -> LoginAction + Send + Sync;
type BoxedCheck = Box<ChecklistFn>;

/// A checklist of functions to run when a player logs in.
#[derive(Resource)]
pub struct LoginChecklist<V: Version> {
    checklist: Vec<BoxedCheck>,
    _phantom: PhantomData<V>,
}

impl<V: Version> Default for LoginChecklist<V> {
    fn default() -> Self { Self::new() }
}

impl<V: Version> LoginChecklist<V> {
    /// Create a new empty [`LoginChecklist`].
    #[must_use]
    pub const fn new_empty() -> Self { Self { checklist: Vec::new(), _phantom: PhantomData } }

    /// Create a new [`LoginChecklist`] with the default checks.
    #[must_use]
    pub fn new() -> Self {
        let mut checklist = Self::new_empty();
        // checklist.add(LoginCompressionAction::set_compression);
        checklist.add(LoginProfileAction::has_profile);
        checklist
    }

    /// Add a function to the checklist.
    pub fn add(&mut self, f: impl Fn(Entity, &World) -> LoginAction + Send + Sync + 'static) {
        self.checklist.push(Box::new(f));
    }

    /// Add a function to the checklist.
    pub fn add_boxed(&mut self, f: BoxedCheck) { self.checklist.push(f); }

    /// Check if an [`Entity`] should be allowed to login.
    #[must_use]
    pub fn check(&self, entity: Entity, world: &World) -> LoginAction {
        self.checklist
            .iter()
            .map(|f| f(entity, world))
            .find(|action| matches!(action, LoginAction::Deny(_)))
            .unwrap_or(LoginAction::Accept)
    }
}

/// An action to take with a login request.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoginAction {
    /// Accept the login request.
    Accept,
    /// Deny the login request with an optional reason.
    Deny(Option<String>),
}
