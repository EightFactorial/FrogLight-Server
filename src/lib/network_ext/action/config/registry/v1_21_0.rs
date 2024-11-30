use std::sync::LazyLock;

use froglight::{
    network::versions::v1_21_0::{configuration::DynamicRegistriesPacket, V1_21_0},
    prelude::{RegistryData, ResourceKey},
};

use super::ConfigRegistryTrait;
use crate::{network::ConfigTask, world::DimensionList};

impl ConfigRegistryTrait for V1_21_0 {
    fn send_registries(dimensions: &DimensionList, task: &ConfigTask<Self>) {
        // Send `minecraft:dimension_type`
        task.send(DynamicRegistriesPacket {
            identifier: ResourceKey::const_new("minecraft:dimension_type"),
            registry_data: dimensions
                .iter()
                .map(|d| RegistryData { identifier: d.dimension_key.clone(), data: None })
                .collect(),
        });

        // Send `minecraft:painting_variant`
        task.send(PAINTING_VARIANT.clone());
        // Send `minecraft:wolf_variant`
        task.send(WOLF_VARIANT.clone());
        // Send `minecraft:worldgen/biome`
        task.send(WORLDGEN_BIOME.clone());
        // Send `minecraft:damage_type`
        task.send(DAMAGE_TYPE.clone());
    }
}

// TODO: Don't hardcode regitries

static WOLF_VARIANT: LazyLock<DynamicRegistriesPacket> =
    LazyLock::new(|| DynamicRegistriesPacket {
        identifier: ResourceKey::const_new("minecraft:wolf_variant"),
        registry_data: vec![
            RegistryData { identifier: ResourceKey::const_new("minecraft:ashen"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:black"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:chestnut"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:pale"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:rusty"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:snowy"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:spotted"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:striped"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:woods"), data: None },
        ],
    });

static PAINTING_VARIANT: LazyLock<DynamicRegistriesPacket> =
    LazyLock::new(|| DynamicRegistriesPacket {
        identifier: ResourceKey::const_new("minecraft:painting_variant"),
        registry_data: vec![
            RegistryData { identifier: ResourceKey::const_new("minecraft:alban"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:aztec"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:aztec2"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:backyard"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:baroque"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:bomb"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:bouquet"), data: None },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:burning_skull"),
                data: None,
            },
            RegistryData { identifier: ResourceKey::const_new("minecraft:bust"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:cavebird"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:changing"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:cotan"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:courbet"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:creebet"), data: None },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:donkey_kong"),
                data: None,
            },
            RegistryData { identifier: ResourceKey::const_new("minecraft:earth"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:endboss"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:fern"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:fighters"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:finding"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:fire"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:graham"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:humble"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:kebab"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:lowmist"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:match"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:meditative"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:orb"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:owlemons"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:passage"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:pigscene"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:plant"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:pointer"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:pond"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:pool"), data: None },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:prairie_ride"),
                data: None,
            },
            RegistryData { identifier: ResourceKey::const_new("minecraft:sea"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:skeleton"), data: None },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:skull_and_roses"),
                data: None,
            },
            RegistryData { identifier: ResourceKey::const_new("minecraft:stage"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:sunflowers"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:sunset"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:tides"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:unpacked"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:void"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:wanderer"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:wasteland"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:water"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:wind"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:wither"), data: None },
        ],
    });

static WORLDGEN_BIOME: LazyLock<DynamicRegistriesPacket> =
    LazyLock::new(|| DynamicRegistriesPacket {
        identifier: ResourceKey::const_new("minecraft:worldgen/biome"),
        registry_data: vec![
            RegistryData { identifier: ResourceKey::const_new("minecraft:badlands"), data: None },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:bamboo_jungle"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:basalt_deltas"),
                data: None,
            },
            RegistryData { identifier: ResourceKey::const_new("minecraft:beach"), data: None },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:birch_forest"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:cherry_grove"),
                data: None,
            },
            RegistryData { identifier: ResourceKey::const_new("minecraft:cold_ocean"), data: None },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:crimson_forest"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:dark_forest"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:deep_cold_ocean"),
                data: None,
            },
            RegistryData { identifier: ResourceKey::const_new("minecraft:deep_dark"), data: None },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:deep_frozen_ocean"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:deep_lukewarm_ocean"),
                data: None,
            },
            RegistryData { identifier: ResourceKey::const_new("minecraft:deep_ocean"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:desert"), data: None },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:dripstone_caves"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:end_barrens"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:end_highlands"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:end_midlands"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:eroded_badlands"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:flower_forest"),
                data: None,
            },
            RegistryData { identifier: ResourceKey::const_new("minecraft:forest"), data: None },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:frozen_ocean"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:frozen_peaks"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:frozen_river"),
                data: None,
            },
            RegistryData { identifier: ResourceKey::const_new("minecraft:grove"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:ice_spikes"), data: None },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:jagged_peaks"),
                data: None,
            },
            RegistryData { identifier: ResourceKey::const_new("minecraft:jungle"), data: None },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:lukewarm_ocean"),
                data: None,
            },
            RegistryData { identifier: ResourceKey::const_new("minecraft:lush_caves"), data: None },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:mangrove_swamp"),
                data: None,
            },
            RegistryData { identifier: ResourceKey::const_new("minecraft:meadow"), data: None },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:mushroom_fields"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:nether_wastes"),
                data: None,
            },
            RegistryData { identifier: ResourceKey::const_new("minecraft:ocean"), data: None },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:old_growth_birch_forest"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:old_growth_pine_taiga"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:old_growth_spruce_taiga"),
                data: None,
            },
            RegistryData { identifier: ResourceKey::const_new("minecraft:plains"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:river"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:savanna"), data: None },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:savanna_plateau"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:small_end_islands"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:snowy_beach"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:snowy_plains"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:snowy_slopes"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:snowy_taiga"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:soul_sand_valley"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:sparse_jungle"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:stony_peaks"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:stony_shore"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:sunflower_plains"),
                data: None,
            },
            RegistryData { identifier: ResourceKey::const_new("minecraft:swamp"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:taiga"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:the_end"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:the_void"), data: None },
            RegistryData { identifier: ResourceKey::const_new("minecraft:warm_ocean"), data: None },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:warped_forest"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:windswept_forest"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:windswept_gravelly_hills"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:windswept_hills"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:windswept_savanna"),
                data: None,
            },
            RegistryData {
                identifier: ResourceKey::const_new("minecraft:wooded_badlands"),
                data: None,
            },
        ],
    });

static DAMAGE_TYPE: LazyLock<DynamicRegistriesPacket> = LazyLock::new(|| DynamicRegistriesPacket {
    identifier: ResourceKey::const_new("minecraft:damage_type"),
    registry_data: vec![
        RegistryData { identifier: ResourceKey::const_new("minecraft:arrow"), data: None },
        RegistryData {
            identifier: ResourceKey::const_new("minecraft:bad_respawn_point"),
            data: None,
        },
        RegistryData { identifier: ResourceKey::const_new("minecraft:cactus"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:campfire"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:cramming"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:dragon_breath"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:drown"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:dry_out"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:explosion"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:fall"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:falling_anvil"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:falling_block"), data: None },
        RegistryData {
            identifier: ResourceKey::const_new("minecraft:falling_stalactite"),
            data: None,
        },
        RegistryData { identifier: ResourceKey::const_new("minecraft:fireball"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:fireworks"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:fly_into_wall"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:freeze"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:generic"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:generic_kill"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:hot_floor"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:in_fire"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:in_wall"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:indirect_magic"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:lava"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:lightning_bolt"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:magic"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:mob_attack"), data: None },
        RegistryData {
            identifier: ResourceKey::const_new("minecraft:mob_attack_no_aggro"),
            data: None,
        },
        RegistryData { identifier: ResourceKey::const_new("minecraft:mob_projectile"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:on_fire"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:out_of_world"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:outside_border"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:player_attack"), data: None },
        RegistryData {
            identifier: ResourceKey::const_new("minecraft:player_explosion"),
            data: None,
        },
        RegistryData { identifier: ResourceKey::const_new("minecraft:sonic_boom"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:spit"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:stalagmite"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:starve"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:sting"), data: None },
        RegistryData {
            identifier: ResourceKey::const_new("minecraft:sweet_berry_bush"),
            data: None,
        },
        RegistryData { identifier: ResourceKey::const_new("minecraft:thorns"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:thrown"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:trident"), data: None },
        RegistryData {
            identifier: ResourceKey::const_new("minecraft:unattributed_fireball"),
            data: None,
        },
        RegistryData { identifier: ResourceKey::const_new("minecraft:wind_charge"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:wither"), data: None },
        RegistryData { identifier: ResourceKey::const_new("minecraft:wither_skull"), data: None },
    ],
});
