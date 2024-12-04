use bevy::prelude::*;
use froglight::{
    network::versions::v1_21_0::{
        play::{
            ChunkRenderDistanceCenterPacket, GameJoinPacket, GameStateChangePacket,
            PlayClientboundPackets, PlayerPositionLookPacket, PlayerSpawnPositionPacket,
        },
        V1_21_0,
    },
    prelude::*,
};

use super::PlayerInitialize;
use crate::{
    dimension::subapp::DimensionIdentifier,
    network::play::PlayServerPacketEvent,
    world::{
        entity::EntityIds,
        positioner::{PlayerPositioner, PositionerDefault},
    },
};

impl PlayerInitialize for V1_21_0 {
    fn initialize(entity: Entity, world: &World) -> Vec<PlayServerPacketEvent<Self>> {
        let entities = world.resource::<EntityIds>();

        let dimension = world.resource::<DimensionIdentifier>();
        let default = world.resource::<PositionerDefault>();
        let positioner = world.resource::<PlayerPositioner>();

        let profile = world.get::<GameProfile>(entity).unwrap();
        let data = positioner.get_or_default(profile.uuid, dimension).unwrap_or_else(|| default);

        vec![
            PlayServerPacketEvent::new(
                entity,
                PlayClientboundPackets::GameJoin(GameJoinPacket {
                    entity_id: entities.next().into(),
                    hardcore: false,
                    dimensions: vec![
                        ResourceKey::const_new("minecraft:the_nether"),
                        ResourceKey::const_new("minecraft:overworld"),
                        ResourceKey::const_new("minecraft:the_end"),
                    ],
                    max_players: 20,
                    view_distance: 12,
                    simulation_distance: 8,
                    reduced_debug_info: false,
                    show_death_screen: false,
                    limited_crafting: false,
                    spawn_info: SpawnInformation {
                        dimension_id: 0,
                        dimension_name: ResourceKey::new("minecraft:overworld"),
                        seed: 0,
                        gamemode: GameMode::Creative,
                        previous_gamemode: -1,
                        debug: false,
                        flat: false,
                        last_death: None,
                        portal_cooldown: 0,
                    },
                    enforce_secure_chat: false,
                }),
            ),
            PlayServerPacketEvent::new(
                entity,
                PlayClientboundPackets::PlayerSpawnPosition(PlayerSpawnPositionPacket {
                    position: data.position,
                    angle: 0.0,
                }),
            ),
            PlayServerPacketEvent::new(
                entity,
                PlayClientboundPackets::PlayerPositionLook(PlayerPositionLookPacket {
                    position: data.position.into(),
                    yaw: 0.0,
                    pitch: 0.0,
                    flags: RelativePositionFlags::default(),
                    teleport_id: 0,
                }),
            ),
            PlayServerPacketEvent::new(
                entity,
                PlayClientboundPackets::ChunkRenderDistanceCenter(
                    ChunkRenderDistanceCenterPacket { chunk_x: 0, chunk_z: 0 },
                ),
            ),
            PlayServerPacketEvent::new(
                entity,
                PlayClientboundPackets::GameStateChange(GameStateChangePacket {
                    event_id: 13,
                    event_data: 0.0,
                }),
            ),
        ]
    }
}
