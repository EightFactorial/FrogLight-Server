use std::sync::Arc;

use bevy::{
    app::InternedAppLabel,
    prelude::{Component, Deref, Resource},
    utils::HashMap,
};
use froglight::{network::connection::NetworkDirection, prelude::*};
use parking_lot::RwLock;

use crate::network::common::{
    ComponentFilter, ConnectionFilter, ConnectionStateEvent, ConnectionTask, PacketEvent,
};

#[expect(missing_docs)]
pub type PlayFilter<V> = ConnectionFilter<V, Play>;
#[expect(missing_docs)]
pub type PlayPacketEvent<V> = PacketEvent<V, Play>;
#[expect(missing_docs)]
pub type PlayRequiredComponents<V> = ComponentFilter<V, Play>;
#[expect(missing_docs)]
pub type PlayStateEvent<V> = ConnectionStateEvent<V, Play>;
#[expect(missing_docs)]
pub type PlayTask<V> = ConnectionTask<V, Play>;

/// A sharable queue of [`PlayPacketEvent`]s, indexed by [`InternedAppLabel`].
#[derive(Clone, Deref, Resource)]
pub struct PlayPacketEventQueue<V: Version>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    queue: Arc<RwLock<HashMap<InternedAppLabel, Vec<PlayPacketEvent<V>>>>>,
}
impl<V: Version> Default for PlayPacketEventQueue<V>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    fn default() -> Self { Self { queue: Arc::new(RwLock::new(HashMap::default())) } }
}

/// A marker component that indicates that the play session has been
/// completed.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
#[component(storage = "SparseSet")]
pub struct CompletedPlay;
