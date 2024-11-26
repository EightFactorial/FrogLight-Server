use bevy::prelude::{Resource, World};

use super::{ConnectionFilter, RateLimitFilter};
use crate::plugin::listen::ConnectionRequest;

/// A filter function that determines whether a connection should be allowed.
pub type FilterFn = dyn Fn(&ConnectionRequest, &World) -> FilterResult + Send + Sync;
/// A boxed [`FilterFn`] that can be stored in the [`ConnectionFilterList`].
pub type BoxedFilter = Box<FilterFn>;

/// A set of filters that determine whether a
/// [`ConnectionRequest`] should be allowed.
#[derive(Resource)]
pub struct ConnectionFilterList {
    filters: Vec<BoxedFilter>,
}

impl Default for ConnectionFilterList {
    fn default() -> Self { Self::new() }
}

impl ConnectionFilterList {
    /// Creates a new [`ConnectionFilterList`] with the default filters.
    #[must_use]
    pub fn new() -> Self {
        let mut filter = Self::new_empty();
        filter.add_filter(ConnectionFilter::filter);
        filter.add_filter(RateLimitFilter::filter);
        filter
    }

    /// Creates a new empty [`ConnectionFilterList`].
    #[must_use]
    pub const fn new_empty() -> Self { Self { filters: Vec::new() } }

    /// Adds a filter to the [`ConnectionFilterList`].
    pub fn add_filter<F>(&mut self, filter: F)
    where
        F: Fn(&ConnectionRequest, &World) -> FilterResult + Send + Sync + 'static,
    {
        self.filters.push(Box::new(filter));
    }

    /// Returns [`FilterResult::Allow`] if the request passes all filters,
    /// or the first [`FilterResult::Deny`] result.
    #[must_use]
    pub fn passes(&self, request: &ConnectionRequest, world: &World) -> FilterResult {
        self.filters
            .iter()
            .map(|filter| filter(request, world))
            .find(|result| *result != FilterResult::Allow)
            .unwrap_or(FilterResult::Allow)
    }
}

/// The result of a [`FilterFn`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FilterResult {
    /// Allow the connection.
    Allow,
    /// Deny the connection with an optional reason.
    Deny(Option<String>),
}
