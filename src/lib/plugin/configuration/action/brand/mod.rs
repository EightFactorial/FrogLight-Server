use bevy::prelude::*;
use compact_str::CompactString;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use crate::{
    configuration::{ConfigAction, ConfigTask, TARGET},
    ConfigPacketEvent,
};

mod v1_21_0;

/// The brand of the server.
///
/// Sent to all connecting clients as the server brand.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct ServerBrand(CompactString);

impl Default for ServerBrand {
    fn default() -> Self { Self(Self::DEFAULT) }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
#[component(storage = "Table")]
pub struct HasServerBrand;

/// The brand of the client.
///
/// Sent to the server when the client connects.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Component)]
pub struct ClientBrand(CompactString);

impl ServerBrand {
    /// The default [`ServerBrand`] for the server.
    pub const DEFAULT: CompactString = CompactString::const_new("froglight");

    /// The [`ResourceKey`] for the server brand.
    const IDENTIFIER: ResourceKey = ResourceKey::const_new("minecraft:brand");

    /// When a client connects, send the server brand to the client.
    #[expect(clippy::type_complexity, private_bounds)]
    pub fn send_server_brand<V: Version + SendServerBrand>(
        query: Query<(Entity, &ConfigTask<V>), (Added<ConfigTask<V>>, Without<HasServerBrand>)>,
        brand: Res<ServerBrand>,
        mut commands: Commands,
    ) where
        Clientbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
        Configuration: State<V>,
        Play: State<V>,
    {
        for (entity, task) in &query {
            trace!(target: TARGET, "Sending server brand: \"{}\"", **brand);

            <V as SendServerBrand>::send_brand(task, &brand);
            commands.entity(entity).insert(HasServerBrand);
        }
    }

    /// When a client connects, receive the client brand from the client.
    #[expect(private_bounds)]
    pub fn recv_client_brand<V: Version + SendServerBrand>(
        query: Query<(Option<&ClientBrand>, Option<&GameProfile>)>,
        mut events: EventReader<ConfigPacketEvent<V>>,
        mut commands: Commands,
    ) where
        Clientbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
        Configuration: State<V>,
        Play: State<V>,
    {
        for event in events.read() {
            if let Some(recv_brand) = <V as SendServerBrand>::recv_brand(&event.packet) {
                match query.get(event.entity) {
                    // Log the received brand.
                    Ok((None, Some(profile))) => {
                        debug!(target: TARGET, "Received brand for {}: \"{recv_brand}\"", profile.name);
                    }
                    // Warn if the client sends a different brand.
                    Ok((Some(brand), Some(profile))) => {
                        if recv_brand == **brand {
                            continue;
                        }
                        warn!(target: TARGET, "Received different brand for {}: \"{}\" -> \"{recv_brand}\"", profile.name, **brand);
                    }
                    // Warn if the client without a profile sends a brand.
                    _ => {
                        warn!(target: TARGET, "Received brand for unknown entity {}", event.entity);
                    }
                }

                // Insert the client's brand.
                commands.entity(event.entity).insert(ClientBrand(recv_brand));
            }
        }
    }

    /// The reason for denying a connection's configuration.
    const DENY_REASON: &'static str = "Server brand was not sent";

    /// A [`ConfigChecklist`](crate::configuration::ConfigChecklist) function
    /// that checks if the server brand has been sent.
    pub(crate) fn has_sent_brand(entity: Entity, world: &World) -> ConfigAction {
        if world.get::<HasServerBrand>(entity).is_some() {
            ConfigAction::Accept
        } else {
            ConfigAction::Deny(Some(Self::DENY_REASON.to_string()))
        }
    }
}

pub(crate) trait SendServerBrand: Version
where
    Clientbound: NetworkDirection<Self, Configuration> + NetworkDirection<Self, Play>,
    Configuration: State<Self>,
    Play: State<Self>,
{
    fn send_brand(task: &ConfigTask<Self>, brand: &ServerBrand);

    fn recv_brand(
        packet: &<Configuration as State<Self>>::ServerboundPacket,
    ) -> Option<CompactString>;
}
