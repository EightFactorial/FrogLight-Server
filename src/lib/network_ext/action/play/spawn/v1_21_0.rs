use bevy::{
    math::DVec3,
    prelude::{EntityCommands, Transform},
};
use froglight::{
    network::versions::v1_21_0::{
        play::{
            ChunkRenderDistanceCenterPacket, GameStateChangePacket, PlayerPositionLookPacket,
            PlayerSpawnPositionPacket,
        },
        V1_21_0,
    },
    prelude::{BlockPosition, ChunkPosition, RelativePositionFlags},
};

use super::PlaySpawnTrait;
use crate::network::PlayTask;

impl PlaySpawnTrait for V1_21_0 {
    fn spawn_client(task: &PlayTask<Self>, commands: &mut EntityCommands) {
        commands.insert(ChunkPosition::splat(0));
        commands.insert(Transform::from_xyz(0.0, 300.0, 0.0));

        task.send(PlayerSpawnPositionPacket {
            position: BlockPosition::new(0, 128, 0),
            angle: 0.0,
        });

        task.send(PlayerPositionLookPacket {
            position: DVec3::new(0.0, 300.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            flags: RelativePositionFlags::default(),
            teleport_id: 0,
        });

        task.send(ChunkRenderDistanceCenterPacket { chunk_x: 0, chunk_z: 0 });
        task.send(GameStateChangePacket { event_id: 13, event_data: 0. });
    }
}
