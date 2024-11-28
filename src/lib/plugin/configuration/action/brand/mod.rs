use bevy::prelude::*;
use compact_str::CompactString;
use froglight::{
    network::{connection::NetworkDirection, versions::v1_21_0::V1_21_0},
    prelude::{State, *},
};

use crate::login::{LoginPacketEvent, LoginTask};

/// The brand of the server.
///
/// Sent to all connecting clients as the server brand.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct ServerBrand(CompactString);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
#[component(storage = "Table")]
struct HasSentBrand;

impl Default for ServerBrand {
    fn default() -> Self { Self(Self::DEFAULT) }
}

impl ServerBrand {
    /// The default [`ServerBrand`] for the server.
    pub const DEFAULT: CompactString = CompactString::const_new("froglight");

    /// When a client connects, send the server brand to the client.
    #[expect(private_bounds)]
    pub fn send_server_brand<V: Version + SendServerBrand>(
        query: Query<&LoginTask<V>>,
        brand: Res<ServerBrand>,
        mut events: EventReader<LoginPacketEvent<V>>,
        mut commands: Commands,
    ) where
        Clientbound: NetworkDirection<V, Login> + NetworkDirection<V, Configuration>,
        Login: State<V>,
        Configuration: State<V>,
    {
        for LoginPacketEvent { entity, packet } in events.read() {
            if let Ok(channel) = query.get(*entity) {
                <V as SendServerBrand>::send_brand(*entity, packet, channel, &brand);
                commands.entity(*entity).insert(HasSentBrand);
            }
        }
    }

    /// A [`LoginChecklist`](crate::login::LoginChecklist) function that
    /// checks if the server brand has been sent.
    #[expect(dead_code)]
    pub(crate) fn has_sent_brand(entity: Entity, world: &World) -> bool {
        world.get::<HasSentBrand>(entity).is_some()
    }
}

trait SendServerBrand: Version
where
    Clientbound: NetworkDirection<Self, Login> + NetworkDirection<Self, Configuration>,
    Login: State<Self>,
    Configuration: State<Self>,
{
    fn send_brand(
        entity: Entity,
        packet: &<Login as State<Self>>::ServerboundPacket,
        channel: &LoginTask<Self>,
        brand: &ServerBrand,
    );
}
