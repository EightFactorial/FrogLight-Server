//! TODO

use std::hash::BuildHasherDefault;

use bevy::{prelude::*, utils::HashMap};
use derive_more::derive::{Debug, From};
use froglight::{network::connection::NetworkDirection, prelude::*};

use crate::{
    dimension::{
        subapp::{DimensionIdentifier, DimensionMarker},
        All, DimensionApp, Overworld,
    },
    network::{login::LoginStateEvent, play::PlayStateEvent},
};

#[doc(hidden)]
pub(super) fn build<V: Version>(app: &mut App)
where
    Clientbound: NetworkDirection<V, Login> + NetworkDirection<V, Play>,
    Login: State<V>,
    Play: State<V>,
{
    app.init_resource::<PlayerPositioner>();
    app.init_dimension_resource::<PlayerPositioner>(All);

    app.insert_resource(PositionerDefault::from(PositionerData {
        position: BlockPosition::ZERO,
        dimension: DimensionIdentifier::from(Overworld),
    }));
    app.in_all_dimensions(|label, app| {
        app.insert_resource(PositionerDefault::from(PositionerData {
            position: BlockPosition::ZERO,
            dimension: DimensionIdentifier::from(label),
        }));
    });

    app.add_systems(
        Update,
        PlayerPositioner::assign_player_dimensions::<V>
            .run_if(on_event::<LoginStateEvent<V>>.or(on_event::<PlayStateEvent<V>>)),
    );
}

/// The default position for new players.
#[derive(Debug, Deref, DerefMut, From, Resource)]
pub struct PositionerDefault(PositionerData);

/// The dimension and location of a new player.
#[derive(Debug, Default, Resource)]
pub struct PlayerPositioner {
    default_data: HashMap<DimensionIdentifier, PositionerData>,
    data: HashMap<Uuid, PositionerData>,
}

/// The dimension and location of a player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PositionerData {
    /// The position of the player.
    pub position: BlockPosition,
    /// The dimension of the player.
    pub dimension: DimensionIdentifier,
}

impl PlayerPositioner {
    /// Create a new [`PlayerPositioner`] with the given default dimension.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            default_data: HashMap::with_hasher(BuildHasherDefault::new()),
            data: HashMap::with_hasher(BuildHasherDefault::new()),
        }
    }

    /// Set the default position for new players in the given dimension.
    pub fn set_default(
        &mut self,
        position: BlockPosition,
        dimension: impl Into<DimensionIdentifier>,
    ) -> Option<PositionerData> {
        let dimension = dimension.into();
        self.default_data.insert(dimension, PositionerData { position, dimension })
    }

    /// Set the position for a player.
    pub fn set(
        &mut self,
        player: Uuid,
        position: BlockPosition,
        dimension: impl Into<DimensionIdentifier>,
    ) -> Option<PositionerData> {
        let dimension = dimension.into();
        self.data.insert(player, PositionerData { position, dimension })
    }

    /// Get the default position for a player.
    #[must_use]
    pub fn get(&self, player: Uuid) -> Option<&PositionerData> { self.data.get(&player) }

    /// Get the default position for a player,
    /// or the default position for the given dimension.
    #[must_use]
    pub fn get_or_default(
        &self,
        player: Uuid,
        dimension: &DimensionIdentifier,
    ) -> Option<&PositionerData> {
        self.get(player).or_else(|| self.default_data.get(dimension))
    }

    /// A system that assigns the player's dimension based on their profile.
    pub fn assign_player_dimensions<V: Version>(
        query: Query<&GameProfile>,
        default: Res<PositionerDefault>,
        positioner: Res<PlayerPositioner>,
        mut login: EventReader<LoginStateEvent<V>>,
        mut reconfig: EventReader<PlayStateEvent<V>>,
        mut cache: Local<Vec<Entity>>,
        mut commands: Commands,
    ) where
        Clientbound: NetworkDirection<V, Login> + NetworkDirection<V, Play>,
        Login: State<V>,
        Play: State<V>,
    {
        cache.extend(login.read().map(|e| e.entity));
        cache.extend(reconfig.read().map(|e| e.entity));

        for entity in cache.drain(..) {
            let Ok(profile) = query.get(entity) else {
                continue;
            };

            let position = positioner.get_or_default(profile.uuid, &default.dimension);
            let position = position.unwrap_or_else(|| &default.0);

            commands.entity(entity).insert(DimensionMarker::from(position.dimension));
        }
    }
}
