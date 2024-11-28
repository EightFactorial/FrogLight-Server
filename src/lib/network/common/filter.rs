use std::marker::PhantomData;

use bevy::prelude::*;
use compact_str::CompactString;
use froglight::prelude::{State, *};

type FilterFn = dyn Fn(Entity, i32, &World) -> FilterResult + Send;

/// A filter that can be applied to a connection.
#[derive(Resource)]
pub struct ConnectionFilter<V: Version, S: State<V>> {
    filters: Vec<Box<FilterFn>>,
    _phantom: PhantomData<(V, S)>,
}

/// The result of one or more filters.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FilterResult {
    /// Allow the connection.
    Allow,
    /// Deny the connection.
    Deny(Option<CompactString>),
}

impl<V: Version, S: State<V>> ConnectionFilter<V, S> {
    /// Create a new empty [`ConnectionFilter`].
    #[inline]
    #[must_use]
    pub const fn new_empty() -> Self { Self { filters: Vec::new(), _phantom: PhantomData } }

    /// Add a [`FilterFn`] to the [`ConnectionFilter`].
    #[inline]
    pub fn add_filter(
        &mut self,
        filter: impl Fn(Entity, i32, &World) -> FilterResult + Send + 'static,
    ) {
        self.filters.push(Box::new(filter));
    }

    /// Add a [`boxed`](Box) [`FilterFn`] to the [`ConnectionFilter`].
    #[inline]
    pub fn add_boxed_filter(&mut self, filter: Box<FilterFn>) { self.filters.push(filter); }

    /// Run the filters on the given [`Entity`].
    #[must_use]
    pub fn filter(&self, entity: Entity, world: &World) -> FilterResult {
        self.filters
            .iter()
            .map(|filter| filter(entity, V::ID, world))
            .find(|result| result != &FilterResult::Allow)
            .unwrap_or(FilterResult::Allow)
    }
}
