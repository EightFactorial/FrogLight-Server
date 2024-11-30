use std::marker::PhantomData;

use bevy::prelude::*;
use compact_str::CompactString;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use crate::{
    network::{ConfigFilter, ConfigPacketEvent, FilterResult},
    network_ext::{NetworkExtConfigSet, TARGET},
};

mod v1_21_0;

/// A [`Plugin`] that receives client configuration.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfigOptionsPlugin<V: Version>(PhantomData<V>);

impl<V: Version + ConfigOptionsTrait> Plugin for ConfigOptionsPlugin<V>
where
    Clientbound: NetworkDirection<V, Configuration>,
    Configuration: State<V>,
{
    fn build(&self, app: &mut App) {
        let mut filters = app.world_mut().resource_mut::<ConfigFilter<V>>();
        filters.add_filter(Self::require_configuration);

        app.add_systems(Update, Self::receive_client_configuration.in_set(NetworkExtConfigSet));
    }
}

/// A [`Component`] that marks the finish packet as already sent.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Component)]
pub struct ClientConfiguration {
    /// The client's language.
    pub language: CompactString,
    /// The clients's view distance.
    pub view_distance: u8,
    /// What kind of chat messages the client sees.
    pub chat_visibility: ChatVisibility,
    /// Whether the client allows chat colors.
    pub chat_colors: bool,
    /// The player's model customization flags.
    pub model_customization: PlayerModelFlags,
    /// The player's main hand.
    pub main_hand: PlayerHand,
    /// Whether the client has text filtering enabled.
    pub text_filtering_enabled: bool,
    /// Whether the client allows appearing in the server listing.
    pub allows_listing: bool,
}

impl<V: Version + ConfigOptionsTrait> ConfigOptionsPlugin<V>
where
    Clientbound: NetworkDirection<V, Configuration>,
    Configuration: State<V>,
{
    /// A system that receives the client's configuration.
    pub fn receive_client_configuration(
        mut query: Query<(&GameProfile, Option<&mut ClientConfiguration>)>,
        mut events: EventReader<ConfigPacketEvent<V>>,
        mut commands: Commands,
    ) {
        for event in events.read() {
            if let Some(new_config) = V::receive_config(&event.packet) {
                if let Ok((profile, current_config)) = query.get_mut(event.entity) {
                    if let Some(mut current_config) = current_config {
                        if *current_config != new_config {
                            debug!(target: TARGET, "Received new client configuration from {}", profile.name);
                        }
                        *current_config = new_config;
                    } else {
                        debug!(target: TARGET, "Received client configuration from {}", profile.name);
                        commands.entity(event.entity).insert(new_config);
                    }
                }
            }
        }
    }

    const DENY_REASON: &'static str = "Client configuration not received";

    /// A filter that denies clients that
    /// have not ben sent a finish packet.
    fn require_configuration(entity: Entity, world: &World) -> FilterResult {
        if world.get::<ClientConfiguration>(entity).is_some() {
            FilterResult::Allow
        } else {
            FilterResult::Deny(Some(Self::DENY_REASON.into()))
        }
    }
}

/// A trait for receiving client configuration.
pub trait ConfigOptionsTrait: Version
where
    Clientbound: NetworkDirection<Self, Configuration>,
    Configuration: State<Self>,
{
    /// Receive the client's configuration.
    fn receive_config(
        packet: &<Configuration as State<Self>>::ServerboundPacket,
    ) -> Option<ClientConfiguration>;
}
