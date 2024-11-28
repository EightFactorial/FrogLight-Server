use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use crate::login::{LoginAction, LoginTask, TARGET};

mod v1_21_0;

/// An action that sends the [`GameProfile`] to connecting clients.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoginProfileAction;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
#[component(storage = "Table")]
struct WasSentLogin;

impl LoginProfileAction {
    /// A system that sends the [`GameProfile`] to connecting clients.
    #[expect(clippy::type_complexity, private_bounds)]
    pub fn send_login_profile<V: Version + SendProfile>(
        query: Query<(Entity, &GameProfile, &LoginTask<V>), Added<LoginTask<V>>>,
        mut commands: Commands,
    ) where
        Clientbound: NetworkDirection<V, Login> + NetworkDirection<V, Configuration>,
        Login: State<V>,
        Configuration: State<V>,
    {
        for (entity, profile, task) in &query {
            debug!(target: TARGET, "Sent profile to {}", profile.name);

            <V as SendProfile>::send_profile(profile, task);
            commands.entity(entity).insert(WasSentLogin);
        }
    }

    /// The reason for denying a login if the profile was not sent.
    const DENY_REASON: &'static str = "Profile was not sent";

    /// Check if the [`GameProfile`] was sent.
    pub fn has_profile(entity: Entity, world: &World) -> LoginAction {
        if world.get::<WasSentLogin>(entity).is_some() {
            LoginAction::Accept
        } else {
            LoginAction::Deny(Some(Self::DENY_REASON.to_string()))
        }
    }
}

pub(crate) trait SendProfile: Version
where
    Clientbound: NetworkDirection<Self, Login> + NetworkDirection<Self, Configuration>,
    Login: State<Self>,
    Configuration: State<Self>,
{
    fn send_profile(profile: &GameProfile, task: &LoginTask<Self>);
}
