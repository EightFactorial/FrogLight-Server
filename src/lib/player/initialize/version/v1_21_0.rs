use bevy::{math::DVec3, prelude::*};
use froglight::{
    network::versions::v1_21_0::{
        play::{
            ChunkRenderDistanceCenterPacket, GameJoinPacket, GameStateChangePacket,
            PlayClientboundPackets, PlayerPositionLookPacket, PlayerSpawnPositionPacket,
        },
        V1_21_0,
    },
    prelude::{ChunkPosition, EntityId, GameProfile, RelativePositionFlags, SpawnInformation},
    world::Chunk,
};

use super::InitializeTrait;
use crate::{
    dimension::{subapp::DimensionIdentifier, DimensionList},
    entity::EntityIds,
    network::play::PlayServerPacketEvent,
    player::{initialize::HasJoinPacket, spawner::PlayerSpawnerArc},
};

impl InitializeTrait for V1_21_0 {
    #[allow(clippy::too_many_lines)]
    fn initialize(entity: Entity, world: &World, commands: &mut Commands) {
        let mut entity_com = commands.entity(entity);
        let entity_ref = world.entity(entity);

        let Some(profile) = entity_ref.get::<GameProfile>() else {
            warn!("Failed to initialize player: No GameProfile found!");
            return;
        };

        // Get the current dimension

        let identifier = world.resource::<DimensionIdentifier>();
        let dimensions = world.resource::<DimensionList>();

        let Some(current) = dimensions.get(**identifier) else {
            warn!("Failed to initialize player: Unknown Dimension!");
            return;
        };

        #[expect(clippy::cast_sign_loss)]
        let dimension_id = current.dimension_id as u32;
        let dimension_name = current.dimension_key.clone();

        // Get or create an EntityId for the player

        let entity_id = entity_ref
            .get::<EntityId>()
            .copied()
            .unwrap_or_else(|| world.resource::<EntityIds>().create());

        // Get the player spawnpoint

        let spawner = world.resource::<PlayerSpawnerArc>();
        let spawn = *spawner.write().get_or_set_default(profile.uuid);
        let (block_pos, gamemode) = (spawn.position, spawn.game_mode);

        entity_com.insert(Transform::from_translation(Vec3::from(block_pos)));

        let chunk_pos = ChunkPosition::new(
            block_pos.x() / i64::from(Chunk::WIDTH),
            block_pos.z() / i64::from(Chunk::DEPTH),
        );
        entity_com.insert(chunk_pos);

        let has_join = entity_ref.contains::<HasJoinPacket>();

        if !has_join {
            // Send the initial game join packet
            entity_com.insert(HasJoinPacket);

            // TODO: Get this information from a resource
            commands.send_event(PlayServerPacketEvent::new(
                entity,
                PlayClientboundPackets::GameJoin(GameJoinPacket {
                    entity_id: entity_id.into(),
                    hardcore: false,
                    dimensions: dimensions.iter().map(|d| d.dimension_key.clone()).collect(),
                    max_players: 20,
                    view_distance: 12,
                    simulation_distance: 8,
                    reduced_debug_info: false,
                    show_death_screen: false,
                    limited_crafting: false,
                    spawn_info: SpawnInformation {
                        dimension_id,
                        dimension_name,
                        seed: 0,
                        gamemode,
                        previous_gamemode: -1,
                        debug: false,
                        flat: false,
                        last_death: None,
                        portal_cooldown: 0,
                    },
                    enforce_secure_chat: false,
                }),
            ));
        }

        // Send the extra player packets

        commands.send_event(PlayServerPacketEvent::new(
            entity,
            PlayClientboundPackets::PlayerPositionLook(PlayerPositionLookPacket {
                position: DVec3::from(block_pos),
                yaw: 0.0,
                pitch: 0.0,
                flags: RelativePositionFlags::default(),
                teleport_id: 0,
            }),
        ));

        if !has_join {
            commands.send_event(PlayServerPacketEvent::new(
                entity,
                PlayClientboundPackets::PlayerSpawnPosition(PlayerSpawnPositionPacket {
                    position: block_pos,
                    angle: 0.0,
                }),
            ));

            commands.send_event(PlayServerPacketEvent::new(
                entity,
                PlayClientboundPackets::GameStateChange(GameStateChangePacket {
                    event_id: 13,
                    event_data: 0.,
                }),
            ));
        }

        #[expect(clippy::cast_sign_loss)]
        #[expect(clippy::cast_possible_truncation)]
        commands.send_event(PlayServerPacketEvent::new(
            entity,
            PlayClientboundPackets::ChunkRenderDistanceCenter(ChunkRenderDistanceCenterPacket {
                chunk_x: chunk_pos.x() as u32,
                chunk_z: chunk_pos.z() as u32,
            }),
        ));
    }
}
