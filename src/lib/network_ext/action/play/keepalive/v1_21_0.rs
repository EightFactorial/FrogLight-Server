use froglight::network::versions::v1_21_0::{
    play::{KeepAliveS2CPacket, PlayServerboundPackets},
    V1_21_0,
};

use super::PlayKeepAliveTrait;
use crate::network::PlayTask;

impl PlayKeepAliveTrait for V1_21_0 {
    fn send_keepalive(keepalive: u64, task: &PlayTask<Self>) {
        task.send(KeepAliveS2CPacket { time: keepalive });
    }

    fn recv_keepalive(packet: &PlayServerboundPackets) -> Option<u64> {
        if let PlayServerboundPackets::KeepAlive(packet) = packet {
            Some(packet.time)
        } else {
            None
        }
    }
}
