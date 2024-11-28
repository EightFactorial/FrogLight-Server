use compact_str::CompactString;
use froglight::{
    network::versions::v1_21_0::{
        configuration::{ConfigurationServerboundPackets, SelectKnownPacksS2CPacket},
        V1_21_0,
    },
    prelude::KnownResourcePack,
};

use super::{ClientKnownPacks, KnownPacks};
use crate::configuration::ConfigTask;

impl super::KnownPacksConfig for V1_21_0 {
    const DEFAULT_PACKS: &'static [KnownResourcePack] = &[KnownResourcePack {
        namespace: CompactString::const_new("minecraft"),
        id: CompactString::const_new("core"),
        version: CompactString::const_new("1.21.1"),
    }];

    fn send_packs(task: &ConfigTask<Self>, packs: &KnownPacks<Self>) {
        task.send(SelectKnownPacksS2CPacket { resourcepacks: packs.resourcepacks.clone() });
    }

    fn recv_packs(packet: &ConfigurationServerboundPackets) -> Option<ClientKnownPacks> {
        if let ConfigurationServerboundPackets::SelectKnownPacks(packet) = packet {
            Some(ClientKnownPacks(packet.resourcepacks.clone()))
        } else {
            None
        }
    }
}
