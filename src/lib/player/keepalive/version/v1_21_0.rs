use froglight::network::versions::v1_21_0::{
    configuration::ConfigurationServerboundPackets,
    play::{KeepAliveS2CPacket, PlayServerboundPackets},
    V1_21_0,
};

use super::KeepAliveTrait;
use crate::{
    network::{
        config::{ConfigPacketEvent, ConfigTask},
        play::{PlayClientPacketEvent, PlayTask},
    },
    player::keepalive::KeepAliveCounter,
};

impl KeepAliveTrait for V1_21_0 {
    fn send_config(keepalive: &mut KeepAliveCounter, task: &ConfigTask<Self>) {
        task.send(KeepAliveS2CPacket { time: keepalive.next_keepalive() });
    }

    fn send_play(keepalive: &mut KeepAliveCounter, task: &PlayTask<Self>) {
        task.send(KeepAliveS2CPacket { time: keepalive.next_keepalive() });
    }

    fn recv_config(
        keepalive: &mut KeepAliveCounter,
        event: &ConfigPacketEvent<Self>,
    ) -> Option<bool> {
        if let ConfigurationServerboundPackets::KeepAlive(packet) = &*event.packet {
            Some(keepalive.receive_keepalive(packet.time))
        } else {
            None
        }
    }

    fn recv_play(
        keepalive: &mut KeepAliveCounter,
        event: &PlayClientPacketEvent<Self>,
    ) -> Option<bool> {
        if let PlayServerboundPackets::KeepAlive(packet) = &*event.packet {
            Some(keepalive.receive_keepalive(packet.time))
        } else {
            None
        }
    }
}
