use froglight::prelude::*;

use crate::network::{ConnectionFilter, ConnectionStateEvent, ConnectionTask, PacketEvent};

/// Filters applied to [`Connection`]s in the [`Login`] state.
pub type LoginFilter<V> = ConnectionFilter<V, Login>;
/// A packet from a [`Connection`] in the [`Login`] state.
pub type LoginPacketEvent<V> = PacketEvent<V, Login>;
/// A state event from a [`Connection`] in the [`Login`] state.
pub type LoginStateEvent<V> = ConnectionStateEvent<V, Login>;
/// A task for a [`Connection`] in the [`Login`] state.
pub type LoginTask<V> = ConnectionTask<V, Login>;
