use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use super::ConnectionRequest;
use crate::network::common::FilterResult;

/// A set of filters that determine if a connection request is allowed.
#[derive(Default, Resource)]
pub struct SocketFilter<V: Version>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    #[expect(clippy::type_complexity)]
    functions: Vec<Box<dyn Fn(&ConnectionRequest<V>, &World) -> FilterResult + Send + Sync>>,
}

impl<V: Version> SocketFilter<V>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    /// Create a new empty [`SocketFilter`].
    #[must_use]
    pub const fn new_empty() -> Self { Self { functions: Vec::new() } }

    /// Add a filter to the [`SocketFilter`].
    pub fn add_filter<
        F: Fn(&ConnectionRequest<V>, &World) -> FilterResult + Send + Sync + 'static,
    >(
        &mut self,
        function: F,
    ) {
        self.functions.push(Box::new(function));
    }

    /// Add a boxed filter to the [`SocketFilter`].
    #[expect(clippy::type_complexity)]
    pub fn add_boxed_filter(
        &mut self,
        function: Box<dyn Fn(&ConnectionRequest<V>, &World) -> FilterResult + Send + Sync>,
    ) {
        self.functions.push(function);
    }

    /// Check if the connection request is allowed.
    #[must_use]
    pub fn check(&self, request: &ConnectionRequest<V>, world: &World) -> FilterResult {
        self.functions
            .iter()
            .map(|f| f(request, world))
            .find(|r| r != &FilterResult::Allow)
            .unwrap_or(FilterResult::Allow)
    }
}
