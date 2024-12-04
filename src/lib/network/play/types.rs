use std::sync::Arc;

use bevy::{
    app::InternedAppLabel,
    prelude::{Component, Entity, Event, Resource},
    utils::HashMap,
};
use froglight::{network::connection::NetworkDirection, prelude::*};
use parking_lot::Mutex;

use crate::network::common::{
    ComponentFilter, ConnectionFilter, ConnectionStateEvent, ConnectionTask, PacketEvent,
};

#[expect(missing_docs)]
pub type PlayFilter<V> = ConnectionFilter<V, Play>;
#[expect(missing_docs)]
pub type PlayRequiredComponents<V> = ComponentFilter<V, Play>;
#[expect(missing_docs)]
pub type PlayStateEvent<V> = ConnectionStateEvent<V, Play>;
#[expect(missing_docs)]
pub type PlayTask<V> = ConnectionTask<V, Play>;

#[expect(missing_docs)]
pub type PlayClientPacketEvent<V> = PacketEvent<V, Play>;

/// An [`Event`] that is sent when a packet is received.
#[derive(Clone, Event)]
pub struct PlayServerPacketEvent<V: Version>
where
    Play: State<V>,
{
    /// The [`Entity`] the packet is associated with.
    pub entity: Entity,
    /// The packet.
    pub packet: Arc<<Play as State<V>>::ClientboundPacket>,
}
impl<V: Version> PlayServerPacketEvent<V>
where
    Play: State<V>,
{
    /// Create a new [`PlayServerPacketEvent`].
    #[inline]
    #[must_use]
    pub fn new(entity: Entity, packet: <Play as State<V>>::ClientboundPacket) -> Self {
        Self { entity, packet: Arc::new(packet) }
    }
}

/// A sharable queue of packets, indexed by [`InternedAppLabel`].
#[derive(Clone, Resource)]
pub struct PlayPacketEventQueue<V: Version>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    /// The clientbound packet queue.
    pub client: Arc<Mutex<HashMap<InternedAppLabel, Vec<PlayClientPacketEvent<V>>>>>,
    /// The serverbound packet queue.
    pub server: Arc<Mutex<Vec<PlayServerPacketEvent<V>>>>,
}
impl<V: Version> Default for PlayPacketEventQueue<V>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    fn default() -> Self {
        Self {
            client: Arc::new(Mutex::new(HashMap::default())),
            server: Arc::new(Mutex::new(Vec::default())),
        }
    }
}

/// A marker component that indicates that the play session should be
/// reconfigured.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
#[component(storage = "SparseSet")]
pub struct ShouldReconfigure;

/// A marker component that indicates that the play session has been
/// completed.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
#[component(storage = "SparseSet")]
pub struct CompletedPlay;
