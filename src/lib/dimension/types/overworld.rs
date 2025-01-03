use std::ops::RangeInclusive;

use bevy::{
    app::{AppLabel, InternedAppLabel},
    reflect::Reflect,
};
use compact_str::CompactString;
use froglight::prelude::ResourceKey;

use crate::dimension::{DimensionTrait, MonsterSpawnLightLevel, ReflectDimension};

/// The overworld dimension.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, AppLabel)]
#[reflect(Dimension)]
pub struct Overworld;

impl DimensionTrait for Overworld {
    fn app_label() -> Option<InternedAppLabel> { Some(Self.intern()) }

    const DIMENSION_KEY: ResourceKey = ResourceKey::const_new("minecraft:overworld");
    const DIMENSION_ID: i32 = 0;

    const AMBIENT_LIGHT: f32 = 0.0;
    const BED_WORKS: bool = true;
    const COORDINATE_SCALE: f64 = 1.0;
    const EFFECTS: Option<CompactString> = Some(CompactString::const_new("minecraft:overworld"));
    const FIXED_TIME: Option<f64> = None;
    const HAS_CEILING: bool = false;
    const HAS_RAIDS: bool = true;
    const HAS_SKYLIGHT: bool = true;
    const HEIGHT: i32 = 384;
    const INFINIBURN: CompactString = CompactString::const_new("#minecraft:infiniburn_overworld");
    const LOGICAL_HEIGHT: i32 = 384;
    const MIN_Y: i32 = -64;
    const MONSTER_SPAWN_LIGHT_LEVEL: MonsterSpawnLightLevel =
        MonsterSpawnLightLevel::Uniform(RangeInclusive::new(0, 7));
    const MONSTER_SPAWN_BLOCK_LIGHT_LIMIT: i32 = 0;
    const NATURAL: bool = true;
    const PIGLIN_SAFE: bool = false;
    const RESPAWN_ANCHOR_WORKS: bool = false;
    const ULTRAWARM: bool = false;
}
