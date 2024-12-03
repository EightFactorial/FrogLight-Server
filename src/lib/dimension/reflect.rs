use std::ops::RangeInclusive;

use bevy::{
    app::{AppLabel, InternedAppLabel},
    reflect::FromType,
};
use compact_str::CompactString;
use froglight::prelude::ResourceKey;
use simdnbt::owned::{NbtCompound, NbtTag};

use crate::dimension::subapp::DimensionType;

/// Reflection data for a dimension.
///
/// Any dimensions not registered will not have a [`SubApp`](bevy::app::SubApp)
/// created for them, and will not have systems or plugins added to them when
/// using [`All`](crate::dimension::subapp::All).
#[derive(Debug, Clone, PartialEq)]
#[expect(clippy::struct_excessive_bools)]
pub struct ReflectDimension {
    /// An optional label for the dimension.
    pub app_label: Option<InternedAppLabel>,

    /// The key for the dimension.
    pub dimension_key: ResourceKey,
    /// The ID for the dimension.
    pub dimension_id: i32,

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
    /// The maximum light level monsters spawn at in the dimension.
    pub monster_spawn_block_light_limit: i32,
}

impl ReflectDimension {
    /// Convert the [`ReflectDimension`] to an [`NbtTag`].
    #[inline]
    #[must_use]
    pub fn to_tag(&self) -> NbtTag { NbtTag::Compound(self.to_nbt()) }

    /// Convert the [`ReflectDimension`] to an [`NbtCompound`].
    #[allow(clippy::cast_possible_wrap)]
    #[must_use]
    pub fn to_nbt(&self) -> NbtCompound {
        // Create the compound.
        let mut compound = NbtCompound::from_values(vec![
            ("has_skylight".into(), NbtTag::Byte(i8::from(self.has_skylight))),
            ("has_ceiling".into(), NbtTag::Byte(i8::from(self.has_ceiling))),
            ("ultrawarm".into(), NbtTag::Byte(i8::from(self.ultrawarm))),
            ("natural".into(), NbtTag::Byte(i8::from(self.natural))),
            ("coordinate_scale".into(), NbtTag::Double(self.coordinate_scale)),
            ("bed_works".into(), NbtTag::Byte(i8::from(self.bed_works))),
            ("respawn_anchor_works".into(), NbtTag::Byte(i8::from(self.respawn_anchor_works))),
            ("min_y".into(), NbtTag::Int(self.min_y)),
            ("height".into(), NbtTag::Int(self.height)),
            ("logical_height".into(), NbtTag::Int(self.logical_height)),
            ("infiniburn".into(), NbtTag::String(self.infiniburn.to_string().into())),
            ("ambient_light".into(), NbtTag::Float(self.ambient_light)),
            ("piglin_safe".into(), NbtTag::Byte(i8::from(self.piglin_safe))),
            ("has_raids".into(), NbtTag::Byte(i8::from(self.has_raids))),
            ("monster_spawn_light_level".into(), self.monster_spawn_light_level.to_tag()),
            (
                "monster_spawn_block_light_limit".into(),
                NbtTag::Int(self.monster_spawn_block_light_limit),
            ),
        ]);

        // Add the fixed time if it exists.
        if let Some(fixed_time) = self.fixed_time {
            compound.insert("fixed_time", NbtTag::Double(fixed_time));
        }

        // Add the effects if they exist.
        if let Some(effects) = &self.effects {
            compound.insert("effects", NbtTag::String(effects.to_string().into()));
        }

        compound
    }
}

/// A trait for dimensions.
pub trait DimensionTrait: 'static {
    /// An optional label for the dimension.
    fn app_label() -> Option<InternedAppLabel>;

    /// The key for the dimension.
    const DIMENSION_KEY: ResourceKey;
    /// The ID for the dimension.
    const DIMENSION_ID: i32;

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
    /// The maximum light level monsters spawn at in the dimension.
    const MONSTER_SPAWN_BLOCK_LIGHT_LIMIT: i32;
}

/// The light level monsters spawn at in a dimension.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MonsterSpawnLightLevel {
    /// Monsters spawn at a constant light level.
    Constant(u8),
    /// Monsters spawn at a light level within a range.
    Uniform(RangeInclusive<i32>),
}

impl MonsterSpawnLightLevel {
    /// Convert the [`MonsterSpawnLightLevel`] to an [`NbtTag`].
    #[inline]
    #[must_use]
    pub fn to_tag(&self) -> NbtTag { NbtTag::Compound(self.to_nbt()) }

    /// Convert the [`MonsterSpawnLightLevel`] to an [`NbtCompound`].
    #[must_use]
    pub fn to_nbt(&self) -> NbtCompound {
        let mut compound = NbtCompound::new();
        match self {
            MonsterSpawnLightLevel::Constant(level) => {
                compound.insert("type", NbtTag::String("constant".into()));
                compound.insert("value", NbtTag::Int(i32::from(*level)));
            }
            MonsterSpawnLightLevel::Uniform(range) => {
                compound.insert("type", NbtTag::String("uniform".into()));
                compound.insert("min_inclusive", NbtTag::Int(*range.start()));
                compound.insert("max_inclusive", NbtTag::Int(*range.end()));
            }
        }
        compound
    }
}

impl<D: DimensionTrait + AppLabel + Default> DimensionType for D {}
impl<D: DimensionTrait> FromType<D> for ReflectDimension {
    fn from_type() -> Self {
        ReflectDimension {
            app_label: D::app_label(),
            dimension_key: D::DIMENSION_KEY,
            dimension_id: D::DIMENSION_ID,
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
            monster_spawn_block_light_limit: D::MONSTER_SPAWN_BLOCK_LIGHT_LIMIT,
        }
    }
}
impl<D: DimensionTrait> From<D> for ReflectDimension
where
    ReflectDimension: FromType<D>,
{
    fn from(_: D) -> Self { ReflectDimension::from_type() }
}
