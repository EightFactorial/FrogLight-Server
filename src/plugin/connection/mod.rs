//! TODO

use bevy::prelude::*;

mod filter;
pub use filter::{ConnectionFilter, FilterEntry, FilterMode};

mod filter_list;
pub use filter_list::{BoxedFilter, ConnectionFilterList, FilterFn, FilterResult};

mod ratelimit;
pub use ratelimit::RateLimitFilter;

mod request;

/// The log target for this module.
static TARGET: &str = "NET";

/// A plugin that manages connections to the server.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConnectionFilterList>();
        app.init_resource::<ConnectionFilter>();
        app.init_resource::<RateLimitFilter>();

        // TODO: Find a proper place to schedule these
        app.add_systems(
            PostUpdate,
            (request::poll_connection_requests, RateLimitFilter::tick_ratelimit),
        );
    }
}
