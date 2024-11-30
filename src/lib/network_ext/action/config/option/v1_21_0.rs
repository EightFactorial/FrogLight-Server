use compact_str::ToCompactString;
use froglight::network::versions::v1_21_0::{
    configuration::ConfigurationServerboundPackets, V1_21_0,
};

use super::{ClientConfiguration, ConfigOptionsTrait};

impl ConfigOptionsTrait for V1_21_0 {
    fn receive_config(packet: &ConfigurationServerboundPackets) -> Option<ClientConfiguration> {
        if let ConfigurationServerboundPackets::ClientOptions(packet) = packet {
            Some(ClientConfiguration {
                language: packet.language.to_compact_string(),
                view_distance: packet.view_distance,
                chat_visibility: packet.chat_visibility,
                chat_colors: packet.chat_colors,
                model_customization: packet.model_customization,
                main_hand: packet.main_hand,
                text_filtering_enabled: packet.text_filtering_enabled,
                allows_listing: packet.allows_listing,
            })
        } else {
            None
        }
    }
}
