use std::sync::Arc;

use bevy::{
    prelude::{Deref, DerefMut, Resource},
    utils::HashMap,
};
use froglight::{
    network::versions::v1_21_0::V1_21_0,
    prelude::{ServerPlayers, ServerStatus, ServerVersion, Version},
};
use parking_lot::RwLock;

/// A shared [`ServerStatus`] [`Resource`].
///
/// Any status requests will be served with this status.
#[derive(Debug, Clone, Deref, DerefMut, Resource)]
pub struct ServerStatusArc(Arc<RwLock<ServerStatus>>);

impl ServerStatusArc {
    /// Create a new [`ServerStatusArc`] with the given [`ServerStatus`].
    #[must_use]
    pub fn new(status: ServerStatus) -> Self { Self(Arc::new(RwLock::new(status))) }
}

impl Default for ServerStatusArc {
    fn default() -> Self {
        Self::new(ServerStatus {
            description: "A Froglight server".into(),
            favicon: None,
            players: ServerPlayers { max: 20, online: 0, sample: Vec::new() },
            version: ServerVersion { name: "1.21.1".into(), protocol: V1_21_0::ID },
            enforces_secure_chat: Some(false),
            other: HashMap::new(),
        })
    }
}
