use froglight::prelude::*;

use crate::network::{ConnectionFilter, ConnectionStateEvent, ConnectionTask, PacketEvent};

/// Filters applied to [`Connection`]s in the [`Configuration`] state.
pub type ConfigFilter<V> = ConnectionFilter<V, Configuration>;
/// A packet from a [`Connection`] in the [`Configuration`] state.
pub type ConfigPacketEvent<V> = PacketEvent<V, Configuration>;
/// A state event from a [`Connection`] in the [`Configuration`] state.
pub type ConfigStateEvent<V> = ConnectionStateEvent<V, Configuration>;
/// A task for a [`Connection`] in the [`Configuration`] state.
pub type ConfigTask<V> = ConnectionTask<V, Configuration>;
