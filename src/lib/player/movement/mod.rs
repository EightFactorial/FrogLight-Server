//! TODO

use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{network::connection::NetworkDirection, prelude::*};

// use crate::network::play::{PlayClientPacketEvent, PlayTask};

mod version;
pub use version::MovementTrait;

mod systemset;
pub use systemset::MovementSystemSet;

use crate::dimension::{All, DimensionApp, Network};

/// A [`Plugin`] that manages player movement.
#[derive(Debug, Default)]
pub struct PlayerMovementPlugin<V: Version>(PhantomData<V>);

impl<V: Version + MovementTrait> Plugin for PlayerMovementPlugin<V>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    fn build(&self, app: &mut App) { app.in_dimension(All, Self::sub_build); }
}

impl<V: Version + MovementTrait> PlayerMovementPlugin<V>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    fn sub_build(app: &mut SubApp) {
        // Only configure `MovementSystemSet` if it doesn't already exist.
        if !app
            .world()
            .resource::<Schedules>()
            .get(Network)
            .is_some_and(|s| s.graph().contains_set(MovementSystemSet))
        {
            app.configure_sets(Network, MovementSystemSet);
        }

        // TODO: Receive movement from clients

        // TODO: Change player center
    }
}
