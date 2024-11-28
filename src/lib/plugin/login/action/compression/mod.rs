use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use crate::login::{LoginAction, LoginTask, TARGET};

mod v1_21_0;

/// An action that sends the compression to connecting clients.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct LoginCompressionAction(u32);

impl Default for LoginCompressionAction {
    fn default() -> Self { Self(256) }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
#[component(storage = "Table")]
struct WasSendCompression;

impl LoginCompressionAction {
    /// A system that sends the compression to connecting clients.
    #[expect(clippy::type_complexity, private_bounds)]
    pub fn send_login_compression<V: Version + SendCompression>(
        query: Query<(Entity, &GameProfile, &LoginTask<V>), Added<LoginTask<V>>>,
        level: Res<LoginCompressionAction>,
        mut commands: Commands,
    ) where
        Clientbound: NetworkDirection<V, Login> + NetworkDirection<V, Configuration>,
        Login: State<V>,
        Configuration: State<V>,
    {
        for (entity, profile, task) in &query {
            debug!(target: TARGET, "Sent compression to {}", profile.name);

            <V as SendCompression>::send_compression(**level, task);
            commands.entity(entity).insert(WasSendCompression);
        }
    }

    /// The reason for denying a login if the compression was not set.
    const DENY_REASON: &'static str = "Compression threshold was not set";

    /// Check if the compression threshold was set.
    pub fn set_compression(entity: Entity, world: &World) -> LoginAction {
        if world.get::<WasSendCompression>(entity).is_some() {
            LoginAction::Accept
        } else {
            LoginAction::Deny(Some(Self::DENY_REASON.to_string()))
        }
    }
}

pub(crate) trait SendCompression: Version
where
    Clientbound: NetworkDirection<Self, Login> + NetworkDirection<Self, Configuration>,
    Login: State<Self>,
    Configuration: State<Self>,
{
    fn send_compression(threshold: u32, task: &LoginTask<Self>);
}
