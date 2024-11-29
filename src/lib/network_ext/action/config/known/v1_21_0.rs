use compact_str::CompactString;
use froglight::{
    network::versions::v1_21_0::{
        configuration::{ConfigurationServerboundPackets, SelectKnownPacksS2CPacket},
        V1_21_0,
    },
    prelude::KnownResourcePack,
};

use super::{ClientKnownPacks, ConfigPacketTrait};
use crate::network::ConfigTask;

const CORE: KnownResourcePack = KnownResourcePack {
    namespace: CompactString::const_new("minecraft"),
    id: CompactString::const_new("core"),
    version: CompactString::const_new("1.21.1"),
};

impl ConfigPacketTrait for V1_21_0 {
    fn send_packs(task: &ConfigTask<Self>) {
        task.send(SelectKnownPacksS2CPacket { resourcepacks: vec![CORE] });
    }

    fn receive_packs(packet: &ConfigurationServerboundPackets) -> Option<ClientKnownPacks> {
        if let ConfigurationServerboundPackets::SelectKnownPacks(packet) = packet {
            Some(ClientKnownPacks(packet.resourcepacks.clone()))
        } else {
            None
        }
    }
}
