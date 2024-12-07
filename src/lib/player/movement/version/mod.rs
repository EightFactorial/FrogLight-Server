use bevy::prelude::{Commands, Entity, Transform};
use froglight::{network::connection::NetworkDirection, prelude::*};

use super::ClientGrounded;
use crate::network::play::PlayClientPacketEvent;

mod v1_21_0;

/// A trait for managing player movement.
pub trait MovementTrait: Version
where
    Clientbound: NetworkDirection<Self, Play>,
    Play: State<Self>,
{
    /// Change the player's center chunk.
    fn send_center(entity: Entity, chunk: ChunkPosition, commands: &mut Commands);

    /// Send a teleport packet to the client.
    fn send_teleport(
        entity: Entity,
        teleport_id: u32,
        transform: &Transform,
        commands: &mut Commands,
    );

    /// Receive a teleport packet from the client.
    fn recv_teleport(event: &PlayClientPacketEvent<Self>) -> Option<u32>;

    /// Receive movement from clients.
    ///
    /// Takes the player's current [`ClientGrounded`] and [`Transform`] and,
    /// if they moved, returns a new set of components.
    fn receive_movement(
        transform: &Transform,
        grounded: &ClientGrounded,
        event: &PlayClientPacketEvent<Self>,
    ) -> Option<(Transform, ClientGrounded)>;
}
