use bevy::{
    app::{AppLabel, InternedAppLabel},
    reflect::Reflect,
};
use compact_str::CompactString;
use froglight::prelude::ResourceKey;

use crate::dimension::{DimensionTrait, MonsterSpawnLightLevel, ReflectDimension};

/// The nether dimension.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, AppLabel)]
#[reflect(Dimension)]
pub struct Nether;

impl DimensionTrait for Nether {
    fn app_label() -> Option<InternedAppLabel> { Some(Self.intern()) }

    const DIMENSION_KEY: ResourceKey = ResourceKey::const_new("minecraft:the_nether");
    const DIMENSION_ID: i32 = -1;

    const AMBIENT_LIGHT: f32 = 0.1;
    const BED_WORKS: bool = false;
    const COORDINATE_SCALE: f64 = 8.0;
    const EFFECTS: Option<CompactString> = Some(CompactString::const_new("minecraft:the_nether"));
    const FIXED_TIME: Option<f64> = Some(18000.0);
    const HAS_CEILING: bool = true;
    const HAS_RAIDS: bool = false;
    const HAS_SKYLIGHT: bool = false;
    const HEIGHT: i32 = 256;
    const INFINIBURN: CompactString = CompactString::const_new("#minecraft:infiniburn_nether");
    const LOGICAL_HEIGHT: i32 = 128;
    const MIN_Y: i32 = 0;
    const MONSTER_SPAWN_LIGHT_LEVEL: MonsterSpawnLightLevel = MonsterSpawnLightLevel::Constant(7);
    const MONSTER_SPAWN_BLOCK_LIGHT_LIMIT: i32 = 15;
    const NATURAL: bool = false;
    const PIGLIN_SAFE: bool = true;
    const RESPAWN_ANCHOR_WORKS: bool = true;
    const ULTRAWARM: bool = true;
}
