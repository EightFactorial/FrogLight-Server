use bevy::prelude::*;
use froglight::{
    network::versions::v1_21_0::{
        play::{
            ChunkRenderDistanceCenterPacket, PlayClientboundPackets, PlayServerboundPackets,
            PlayerInputPacket, PlayerMoveFullPacket, PlayerMoveLookAndOnGroundPacket,
            PlayerMoveOnGroundOnlyPacket, PlayerMovePositionAndOnGroundPacket,
            PlayerPositionLookPacket,
        },
        V1_21_0,
    },
    prelude::*,
};

use super::MovementTrait;
use crate::{
    network::play::{PlayClientPacketEvent, PlayServerPacketEvent},
    player::movement::ClientGrounded,
};

impl MovementTrait for V1_21_0 {
    fn send_center(entity: Entity, chunk: ChunkPosition, commands: &mut Commands) {
        commands.send_event(PlayServerPacketEvent::<Self>::new(
            entity,
            PlayClientboundPackets::ChunkRenderDistanceCenter(ChunkRenderDistanceCenterPacket {
                chunk_x: chunk.x_i32(),
                chunk_z: chunk.z_i32(),
            }),
        ));
    }

    fn send_teleport(
        entity: Entity,
        teleport_id: u32,
        transform: &Transform,
        commands: &mut Commands,
    ) {
        let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
        commands.send_event(PlayServerPacketEvent::<Self>::new(
            entity,
            PlayClientboundPackets::PlayerPositionLook(PlayerPositionLookPacket {
                position: transform.translation.as_dvec3(),
                yaw,
                pitch,
                flags: RelativePositionFlags::default(),
                teleport_id,
            }),
        ));
    }

    fn recv_teleport(event: &PlayClientPacketEvent<Self>) -> Option<u32> {
        if let PlayServerboundPackets::TeleportConfirm(packet) = event.packet.as_ref() {
            Some(packet.teleport_id)
        } else {
            None
        }
    }

    fn receive_movement(
        transform: &Transform,
        _grounded: &ClientGrounded,
        event: &PlayClientPacketEvent<Self>,
    ) -> Option<(Transform, ClientGrounded)> {
        match event.packet.as_ref() {
            // Set the position and ground state.
            PlayServerboundPackets::PlayerMovePositionAndOnGround(
                PlayerMovePositionAndOnGroundPacket { position, on_ground },
            ) => Some((transform.with_translation(position.as_vec3()), ClientGrounded(*on_ground))),
            // Set the position, rotation, and ground state.
            PlayServerboundPackets::PlayerMoveFull(PlayerMoveFullPacket {
                position,
                yaw,
                pitch,
                on_ground,
            }) => {
                let rotation = Quat::from_euler(EulerRot::YXZ, *yaw, *pitch, 0.0);
                Some((
                    (Transform::from_translation(position.as_vec3()).with_rotation(rotation)),
                    ClientGrounded(*on_ground),
                ))
            }
            // Set the rotation and ground state.
            PlayServerboundPackets::PlayerMoveLookAndOnGround(
                PlayerMoveLookAndOnGroundPacket { yaw, pitch, on_ground },
            ) => {
                let rotation = Quat::from_euler(EulerRot::YXZ, *yaw, *pitch, 0.0);
                Some(((transform.with_rotation(rotation)), ClientGrounded(*on_ground)))
            }
            // Set the ground state.
            PlayServerboundPackets::PlayerMoveOnGroundOnly(PlayerMoveOnGroundOnlyPacket {
                on_ground,
            }) => Some((*transform, ClientGrounded(*on_ground))),
            // Log the velocity, jumping, and shift state.
            PlayServerboundPackets::PlayerInput(PlayerInputPacket {
                velocity,
                flags: PlayerInputFlags { jumping, shift },
            }) => {
                debug!("Velocity: {velocity}, Jumping: {jumping}, Shift: {shift}");
                None
            }
            _ => None,
        }
    }
}
