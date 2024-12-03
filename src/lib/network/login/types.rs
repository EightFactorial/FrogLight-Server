use std::sync::Arc;

use bevy::prelude::{Component, Deref, Resource};
use compact_str::CompactString;
use froglight::prelude::{Login, Version};
use parking_lot::RwLock;

use crate::network::common::{
    ComponentFilter, ConnectionFilter, ConnectionStateEvent, ConnectionTask, PacketEvent,
};

#[expect(missing_docs)]
pub type LoginFilter<V> = ConnectionFilter<V, Login>;
#[expect(missing_docs)]
pub type LoginPacketEvent<V> = PacketEvent<V, Login>;
#[expect(missing_docs)]
pub type LoginRequiredComponents<V> = ComponentFilter<V, Login>;
#[expect(missing_docs)]
pub type LoginStateEvent<V> = ConnectionStateEvent<V, Login>;
#[expect(missing_docs)]
pub type LoginTask<V> = ConnectionTask<V, Login>;

/// The address of the authentication server.
#[derive(Debug, Clone, Deref, Resource)]
pub struct AuthenticationServer<V: Version> {
    #[deref]
    address: Arc<RwLock<Option<CompactString>>>,
    _phantom: std::marker::PhantomData<V>,
}

/// A marker component that indicates that the login process has been completed.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
#[component(storage = "SparseSet")]
pub struct CompletedLogin;

impl<V: Version> From<Option<CompactString>> for AuthenticationServer<V> {
    fn from(address: Option<CompactString>) -> Self {
        Self { address: Arc::new(RwLock::new(address)), _phantom: std::marker::PhantomData }
    }
}
impl<V: Version> From<Option<String>> for AuthenticationServer<V> {
    fn from(address: Option<String>) -> Self {
        Self {
            address: Arc::new(RwLock::new(address.map(CompactString::new))),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<V: Version> From<CompactString> for AuthenticationServer<V> {
    fn from(address: CompactString) -> Self {
        Self { address: Arc::new(RwLock::new(Some(address))), _phantom: std::marker::PhantomData }
    }
}
impl<V: Version> From<String> for AuthenticationServer<V> {
    fn from(address: String) -> Self {
        Self {
            address: Arc::new(RwLock::new(Some(CompactString::new(address)))),
            _phantom: std::marker::PhantomData,
        }
    }
}
