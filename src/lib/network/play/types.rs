use froglight::prelude::*;

use crate::network::{ConnectionFilter, ConnectionStateEvent, ConnectionTask, PacketEvent};

/// Filters applied to [`Connection`]s in the [`Play`] state.
pub type PlayFilter<V> = ConnectionFilter<V, Play>;
/// A packet from a [`Connection`] in the [`Play`] state.
pub type PlayPacketEvent<V> = PacketEvent<V, Play>;
/// A state event from a [`Connection`] in the [`Play`] state.
pub type PlayStateEvent<V> = ConnectionStateEvent<V, Play>;
/// A task for a [`Connection`] in the [`Play`] state.
pub type PlayTask<V> = ConnectionTask<V, Play>;
