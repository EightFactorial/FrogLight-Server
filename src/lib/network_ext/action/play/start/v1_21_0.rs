use froglight::{
    network::versions::v1_21_0::{play::GameJoinPacket, V1_21_0},
    prelude::{EntityId, GameMode, ResourceKey, SpawnInformation},
};

use super::PlayStartTrait;
use crate::{network::PlayTask, world::DimensionList};

const DIMENSION: ResourceKey = ResourceKey::const_new("minecraft:overworld");

impl PlayStartTrait for V1_21_0 {
    fn initialize_client(entity_id: EntityId, dimensions: &DimensionList, task: &PlayTask<Self>) {
        let dimension_id = dimensions.index_of(&DIMENSION).unwrap();

        // TODO: Get this data from a Resource
        task.send(GameJoinPacket {
            entity_id: entity_id.into(),
            hardcore: false,
            dimensions: vec![DIMENSION],
            max_players: 20,
            view_distance: 12,
            simulation_distance: 12,
            reduced_debug_info: false,
            show_death_screen: false,
            limited_crafting: false,
            spawn_info: SpawnInformation {
                dimension_id,
                dimension_name: DIMENSION,
                seed: 0,
                gamemode: GameMode::Creative,
                previous_gamemode: -1,
                debug: false,
                flat: false,
                last_death: None,
                portal_cooldown: 0,
            },
            enforce_secure_chat: false,
        });
    }
}
