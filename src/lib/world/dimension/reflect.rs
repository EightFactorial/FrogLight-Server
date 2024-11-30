use std::ops::RangeInclusive;

use bevy::{prelude::*, reflect::FromType};
use compact_str::CompactString;
use froglight::prelude::ResourceKey;

/// Reflection data for a dimension.
#[derive(Debug, Clone, PartialEq)]
#[expect(clippy::struct_excessive_bools)]
pub struct ReflectDimension {
    /// The key for the dimension.
    pub dimension_key: ResourceKey,

    /// An optional fixed time for the dimension.
    pub fixed_time: Option<f64>,
    /// Whether the dimension has skylight.
    pub has_skylight: bool,
    /// Whether the dimension has a ceiling.
    pub has_ceiling: bool,
    /// Whether the water evaporates in the dimension.
    pub ultrawarm: bool,
    /// Whether compasses spin randomly in the dimension.
    pub natural: bool,
    /// The coordinate scale of the dimension.
    pub coordinate_scale: f64,
    /// Whether beds work in the dimension.
    pub bed_works: bool,
    /// Whether respawn anchors work in the dimension.
    pub respawn_anchor_works: bool,
    /// The minimum Y coordinate of the dimension.
    pub min_y: i32,
    /// The maximum height of the dimension.
    pub height: i32,
    /// The maximum height portals can generate in the dimension.
    pub logical_height: i32,
    /// The block tag to use for infiniburn.
    pub infiniburn: CompactString,
    /// What dimensional effects the dimension has.
    pub effects: Option<CompactString>,
    /// The ambient light level of the dimension.
    pub ambient_light: f32,
    /// Whether piglins transform into zombified piglins in the dimension.
    pub piglin_safe: bool,
    /// Whether the dimension has raids.
    pub has_raids: bool,
    /// What light level monsters spawn at in the dimension.
    pub monster_spawn_light_level: MonsterSpawnLightLevel,
}

impl<D: DimensionTrait> FromType<D> for ReflectDimension {
    fn from_type() -> Self {
        ReflectDimension {
            dimension_key: D::DIMENSION_KEY,
            fixed_time: D::FIXED_TIME,
            has_skylight: D::HAS_SKYLIGHT,
            has_ceiling: D::HAS_CEILING,
            ultrawarm: D::ULTRAWARM,
            natural: D::NATURAL,
            coordinate_scale: D::COORDINATE_SCALE,
            bed_works: D::BED_WORKS,
            respawn_anchor_works: D::RESPAWN_ANCHOR_WORKS,
            min_y: D::MIN_Y,
            height: D::HEIGHT,
            logical_height: D::LOGICAL_HEIGHT,
            infiniburn: D::INFINIBURN,
            effects: D::EFFECTS,
            ambient_light: D::AMBIENT_LIGHT,
            piglin_safe: D::PIGLIN_SAFE,
            has_raids: D::HAS_RAIDS,
            monster_spawn_light_level: D::MONSTER_SPAWN_LIGHT_LEVEL,
        }
    }
}
impl<D: DimensionTrait> From<D> for ReflectDimension
where
    ReflectDimension: FromType<D>,
{
    fn from(_: D) -> Self { ReflectDimension::from_type() }
}

/// A trait for dimensions.
pub trait DimensionTrait: 'static {
    /// The key for the dimension.
    const DIMENSION_KEY: ResourceKey;

    /// An optional fixed time for the dimension.
    const FIXED_TIME: Option<f64>;
    /// Whether the dimension has skylight.
    const HAS_SKYLIGHT: bool;
    /// Whether the dimension has a ceiling.
    const HAS_CEILING: bool;
    /// Whether the water evaporates in the dimension.
    const ULTRAWARM: bool;
    /// Whether compasses spin randomly in the dimension.
    const NATURAL: bool;
    /// The coordinate scale of the dimension.
    const COORDINATE_SCALE: f64;
    /// Whether beds work in the dimension.
    const BED_WORKS: bool;
    /// Whether respawn anchors work in the dimension.
    const RESPAWN_ANCHOR_WORKS: bool;
    /// The minimum Y coordinate of the dimension.
    const MIN_Y: i32;
    /// The maximum height of the dimension.
    const HEIGHT: i32;
    /// The maximum height portals can generate in the dimension.
    const LOGICAL_HEIGHT: i32;
    /// The block tag to use for infiniburn.
    const INFINIBURN: CompactString;
    /// What dimensional effects the dimension has.
    const EFFECTS: Option<CompactString>;
    /// The ambient light level of the dimension.
    const AMBIENT_LIGHT: f32;
    /// Whether piglins transform into zombified piglins in the dimension.
    const PIGLIN_SAFE: bool;
    /// Whether the dimension has raids.
    const HAS_RAIDS: bool;
    /// What light level monsters spawn at in the dimension.
    const MONSTER_SPAWN_LIGHT_LEVEL: MonsterSpawnLightLevel;
}

/// The light level monsters spawn at in a dimension.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum MonsterSpawnLightLevel {
    /// Monsters spawn at a constant light level.
    Constant(u8),
    /// Monsters spawn at a light level within a range.
    Uniform(RangeInclusive<u8>),
}
