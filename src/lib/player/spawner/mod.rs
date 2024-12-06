//! TODO

use std::{marker::PhantomData, sync::Arc};

use bevy::prelude::*;
use froglight::{network::connection::NetworkDirection, prelude::*};
use parking_lot::RwLock;

use crate::{
    dimension::{
        subapp::{DimensionIdentifier, DimensionMarker},
        All, DimensionApp,
    },
    network::config::ConfigStateEvent,
};

mod spawn;
pub use spawn::{PlayerSpawner, PlayerSpawnerData};

mod systemset;
pub use systemset::SpawnerSystemSet;

/// A [`Plugin`] that manages player spawnpoints.
#[derive(Debug, Default)]
pub struct PlayerSpawnerPlugin<V: Version>(PhantomData<V>);

impl<V: Version> Plugin for PlayerSpawnerPlugin<V>
where
    Clientbound: NetworkDirection<V, Configuration>,
    Configuration: State<V>,
{
    fn build(&self, app: &mut App) {
        // Only insert a `PlayerSpawnerArc` if one doesn't already exist.
        if !app.world().contains_resource::<PlayerSpawnerArc>() {
            let spawner = PlayerSpawnerArc::internal_default();
            app.insert_dimension_resource(All, spawner.clone());
            app.insert_resource(spawner);
        }

        // Only configure `SpawnerSystemSet` if it doesn't already exist.
        if !app
            .world()
            .resource::<Schedules>()
            .get(Update)
            .is_some_and(|s| s.graph().contains_set(SpawnerSystemSet))
        {
            app.configure_sets(Update, SpawnerSystemSet);
        }

        app.add_systems(
            Update,
            PlayerSpawnerArc::set_spawn_dimension::<V>
                .run_if(on_event::<ConfigStateEvent<V>>)
                .in_set(SpawnerSystemSet),
        );
    }
}

/// A [`Resource`] that manages player spawns.
///
/// As a shared reference, this resource can be
/// cheaply cloned and accessed in any [`World`].
#[derive(Debug, Clone, Deref, Resource)]
pub struct PlayerSpawnerArc(Arc<RwLock<PlayerSpawner>>);

impl PlayerSpawnerArc {
    /// Create a new [`PlayerSpawnerArc`] with the default spawn point.
    #[must_use]
    pub fn new(
        game_mode: GameMode,
        position: BlockPosition,
        dimension: DimensionIdentifier,
    ) -> Self {
        Self(Arc::new(RwLock::new(PlayerSpawner::new(game_mode, position, dimension))))
    }

    /// Create a new [`PlayerSpawnerArc`] using the
    /// [`internal default`](PlayerSpawner::internal_default) values.
    #[must_use]
    fn internal_default() -> Self { Self::from_spawner(PlayerSpawner::internal_default()) }

    /// Create a new [`PlayerSpawnerArc`] using a [`PlayerSpawner`].
    #[must_use]
    pub fn from_spawner(spawner: PlayerSpawner) -> Self { Self(Arc::new(RwLock::new(spawner))) }
}

impl PlayerSpawnerArc {
    /// A system that sets the spawn dimension for new connections.
    pub fn set_spawn_dimension<V: Version>(
        query: Query<(&GameProfile, Option<&DimensionMarker>)>,
        spawns: Res<PlayerSpawnerArc>,
        mut events: EventReader<ConfigStateEvent<V>>,
        mut commands: Commands,
    ) where
        Clientbound: NetworkDirection<V, Configuration>,
        Configuration: State<V>,
    {
        for ConfigStateEvent { entity, .. } in events.read() {
            if let Ok((profile, None)) = query.get(*entity) {
                let identifier = spawns.read().get_or_default(&profile.uuid).dimension;
                commands.entity(*entity).insert(DimensionMarker(identifier));
            }
        }
    }
}
