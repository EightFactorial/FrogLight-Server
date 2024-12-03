use bevy::prelude::Component;
use froglight::prelude::Configuration;

use crate::network::common::{
    ComponentFilter, ConnectionFilter, ConnectionStateEvent, ConnectionTask, PacketEvent,
};

#[expect(missing_docs)]
pub type ConfigFilter<V> = ConnectionFilter<V, Configuration>;
#[expect(missing_docs)]
pub type ConfigPacketEvent<V> = PacketEvent<V, Configuration>;
#[expect(missing_docs)]
pub type ConfigRequiredComponents<V> = ComponentFilter<V, Configuration>;
#[expect(missing_docs)]
pub type ConfigStateEvent<V> = ConnectionStateEvent<V, Configuration>;
#[expect(missing_docs)]
pub type ConfigTask<V> = ConnectionTask<V, Configuration>;

/// A marker component that indicates that the configuration process has been
/// completed.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
#[component(storage = "SparseSet")]
pub struct CompletedConfig;
