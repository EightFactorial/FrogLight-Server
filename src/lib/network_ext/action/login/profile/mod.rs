use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use crate::{
    network::{FilterResult, LoginFilter, LoginTask},
    network_ext::{NetworkExtLoginSet, TARGET},
};

mod v1_21_0;

/// A [`Plugin`] that adds sending profiles to clients.
///
/// Also prevents clients from logging in if they have not received a profile.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoginProfilePlugin<V: Version>(PhantomData<V>);

impl<V: Version + LoginProfileTrait> Plugin for LoginProfilePlugin<V>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    fn build(&self, app: &mut App) {
        let mut filters = app.world_mut().resource_mut::<LoginFilter<V>>();
        filters.add_filter(Self::require_sent_profile);

        app.add_systems(Update, Self::send_login_profiles.in_set(NetworkExtLoginSet));
    }
}

/// A [`Component`] that marks a [`GameProfile`] as already sent.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
#[component(storage = "SparseSet")]
pub struct WasSentProfile;

impl<V: Version + LoginProfileTrait> LoginProfilePlugin<V>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    /// A system that sends clients [`GameProfile`]s.
    pub fn send_login_profiles(
        query: Query<(Entity, &GameProfile, &LoginTask<V>), Without<WasSentProfile>>,
        mut commands: Commands,
    ) {
        for (entity, profile, task) in query.iter() {
            debug!(target: TARGET, "Sending profile to {}", profile.name);
            V::send_profile(profile.clone(), task);
            commands.entity(entity).insert(WasSentProfile);
        }
    }

    const DENY_REASON: &'static str = "Profile not sent";

    /// A filter that requires a [`GameProfile`] to be sent.
    fn require_sent_profile(entity: Entity, world: &World) -> FilterResult {
        if world.get::<WasSentProfile>(entity).is_some() {
            FilterResult::Allow
        } else {
            FilterResult::Deny(Some(Self::DENY_REASON.into()))
        }
    }
}

/// A trait that allows sending a [`GameProfile`] in the login state.
pub trait LoginProfileTrait: Version
where
    Clientbound: NetworkDirection<Self, Login>,
    Login: State<Self>,
{
    /// Send a [`GameProfile`] to the client.
    fn send_profile(profile: GameProfile, task: &LoginTask<Self>);
}
