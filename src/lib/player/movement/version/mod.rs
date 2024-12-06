use froglight::{network::connection::NetworkDirection, prelude::*};

use crate::network::play::PlayTask;

mod v1_21_0;

/// A trait for managing player movement.
pub trait MovementTrait: Version
where
    Clientbound: NetworkDirection<Self, Play>,
    Play: State<Self>,
{
    /// Initialize a player entity.
    fn send_teleport(task: &PlayTask<Self>);

    /// Change the player's center chunk.
    fn send_center(task: &PlayTask<Self>);

    /// Receive movement from clients.
    fn receive_movement(task: &PlayTask<Self>);
}
