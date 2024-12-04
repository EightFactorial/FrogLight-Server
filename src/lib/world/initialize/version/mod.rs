use bevy::prelude::{Entity, World};
use froglight::prelude::*;

use crate::network::play::PlayServerPacketEvent;

mod v1_21_0;

///  A trait that defines how to initialize the player.
pub trait PlayerInitialize: Version
where
    Play: State<Self>,
{
    /// Initialize the player.
    fn initialize(entity: Entity, world: &World) -> Vec<PlayServerPacketEvent<Self>>;
}
