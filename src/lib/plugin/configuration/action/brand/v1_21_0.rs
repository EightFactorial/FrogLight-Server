use std::io::Cursor;

use bevy::log::error;
use compact_str::CompactString;
use froglight::{
    network::versions::v1_21_0::{
        configuration::ConfigurationServerboundPackets, play::CustomPayloadS2CPacket, V1_21_0,
    },
    prelude::UnsizedBuffer,
    protocol::{FrogRead, FrogWrite},
};

use super::{SendServerBrand, ServerBrand};
use crate::configuration::{ConfigTask, TARGET};

impl SendServerBrand for V1_21_0 {
    fn send_brand(task: &ConfigTask<Self>, brand: &ServerBrand) {
        let mut payload = UnsizedBuffer::new();
        brand.fg_write(&mut payload).unwrap();

        task.send(CustomPayloadS2CPacket { identifier: ServerBrand::IDENTIFIER, payload });
    }

    fn recv_brand(packet: &ConfigurationServerboundPackets) -> Option<CompactString> {
        if let ConfigurationServerboundPackets::CustomPayload(packet) = packet {
            if packet.identifier == ServerBrand::IDENTIFIER {
                match CompactString::fg_read(&mut Cursor::new(packet.payload.as_slice())) {
                    Ok(brand) => return Some(brand),
                    Err(error) => error!(target: TARGET, "Failed to read server brand: {error}"),
                }
            }
        }

        None
    }
}
