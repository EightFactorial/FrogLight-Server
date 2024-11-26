use std::net::IpAddr;

use bevy::{prelude::*, utils::HashMap};
use parking_lot::Mutex;

use super::FilterResult;
use crate::plugin::{connection::TARGET, listen::ConnectionRequest};

/// A filter that rate-limits connections based on their IP address.
#[derive(Debug, Resource)]
pub struct RateLimitFilter {
    addresses: Mutex<HashMap<IpAddr, u32>>,
    timer: Timer,
}

impl Default for RateLimitFilter {
    fn default() -> Self {
        Self {
            addresses: Mutex::new(HashMap::default()),
            timer: Timer::from_seconds(Self::RATELIMIT_TIMER, TimerMode::Repeating),
        }
    }
}

impl RateLimitFilter {
    /// The number of connection attempts before rate-limiting.
    pub const RATELIMIT_ATTEMPTS: u32 = 5;
    /// The maximum number of connection attempts to store.
    ///
    /// Any number of attempts above this will not increase the counter.
    pub const RATELIMIT_ATTEMPTS_MAX: u32 = 10;

    /// The rate-limit timer duration.
    pub const RATELIMIT_TIMER: f32 = 1.0;

    const RATELIMIT_REASON: &str = "Connection rate-limited";

    /// A [`FilterFn`](super::FilterFn) that checks if the connection
    /// is being rate-limited.
    pub fn filter(request: &ConnectionRequest, world: &World) -> FilterResult {
        if let Some(ratelimit) = world.get_resource::<RateLimitFilter>() {
            let mut addresses = ratelimit.addresses.lock();

            // Add an attempt to the counter
            let attempts = addresses.entry(request.socket.ip()).or_default();
            *attempts = attempts.saturating_add(1);

            // Log the current state for this address
            debug!(target: TARGET,
                "Ratelimit {}: {attempts}/{}",
                request.socket.ip(),
                Self::RATELIMIT_ATTEMPTS
            );

            // Allow the connection if the counter is below the limit
            if *attempts < Self::RATELIMIT_ATTEMPTS {
                FilterResult::Allow
            } else {
                FilterResult::Deny(Some(Self::RATELIMIT_REASON.into()))
            }
        } else {
            FilterResult::Allow
        }
    }

    /// A system that reduces the rate-limit counters over time.
    pub fn tick_ratelimit(mut res: ResMut<RateLimitFilter>, time: Res<Time<Real>>) {
        let Self { addresses, timer } = &mut *res;
        if timer.tick(time.delta()).just_finished() {
            let mut addresses = addresses.lock();

            // Reduce all counters by 1 attempt, and keep under `RATELIMIT_ATTEMPTS_MAX`
            for counter in addresses.values_mut() {
                *counter = counter.saturating_sub(1).min(Self::RATELIMIT_ATTEMPTS_MAX);
            }

            // Remove all counters that are at 0 attempts
            addresses.retain(|_, counter| *counter > 0);
        }
    }
}
