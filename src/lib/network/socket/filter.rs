use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use super::ConnectionRequest;
use crate::network::FilterResult;

type FilterFn<V> = dyn Fn(&ConnectionRequest<V>, &World) -> FilterResult + Send + Sync;

/// A filter that can be applied to a connection.
#[derive(Default, Resource)]
pub struct SocketFilter<V: Version>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    filters: Vec<Box<FilterFn<V>>>,
}

impl<V: Version> SocketFilter<V>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    /// Create a new [`SocketFilter`].
    #[inline]
    #[must_use]
    pub const fn new() -> Self { Self { filters: Vec::new() } }

    /// Add a [`FilterFn`] to the [`SocketFilter`].
    #[inline]
    pub fn add_filter(
        &mut self,
        filter: impl Fn(&ConnectionRequest<V>, &World) -> FilterResult + Send + Sync + 'static,
    ) {
        self.filters.push(Box::new(filter));
    }

    /// Add a [`boxed`](Box) [`FilterFn`] to the [`SocketFilter`].
    #[inline]
    pub fn add_boxed_filter(&mut self, filter: Box<FilterFn<V>>) { self.filters.push(filter); }

    /// Run the filters on the given [`Entity`].
    #[must_use]
    pub fn filter(&self, request: &ConnectionRequest<V>, world: &World) -> FilterResult {
        self.filters
            .iter()
            .map(|filter| filter(request, world))
            .find(|result| result != &FilterResult::Allow)
            .unwrap_or(FilterResult::Allow)
    }
}
