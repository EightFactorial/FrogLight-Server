use bevy::prelude::*;
use froglight::{
    network::versions::v1_21_0::{
        play::{
            ChunkRenderDistanceCenterPacket, GameJoinPacket, GameStateChangePacket,
            PlayClientboundPackets, PlayerSpawnPositionPacket,
        },
        V1_21_0,
    },
    prelude::{ChunkPosition, EntityId, GameProfile, SpawnInformation},
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
        // Check if the player has already been initialized
        let entity_ref = world.entity(entity);
        if entity_ref.contains::<HasJoinPacket>() {
            return;
        }

        // Insert `HasJoinPacket` to prevent initializing multiple times
        let mut entity_com = commands.entity(entity);
        entity_com.insert(HasJoinPacket);

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

        let chunk_pos = ChunkPosition::from_block(block_pos);
        entity_com.insert(chunk_pos);

        // Send the initial game join packet

        // TODO: Get this information from a resource
        commands.send_event(PlayServerPacketEvent::<Self>::new(
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

        // Send the extra player packets

        commands.send_event(PlayServerPacketEvent::<Self>::new(
            entity,
            PlayClientboundPackets::PlayerSpawnPosition(PlayerSpawnPositionPacket {
                position: block_pos,
                angle: 0.0,
            }),
        ));

        commands.send_event(PlayServerPacketEvent::<Self>::new(
            entity,
            PlayClientboundPackets::GameStateChange(GameStateChangePacket {
                event_id: 13,
                event_data: 0.,
            }),
        ));

        commands.send_event(PlayServerPacketEvent::<Self>::new(
            entity,
            PlayClientboundPackets::ChunkRenderDistanceCenter(ChunkRenderDistanceCenterPacket {
                chunk_x: chunk_pos.x_i32(),
                chunk_z: chunk_pos.z_i32(),
            }),
        ));
    }
}
