use froglight::network::versions::v1_21_0::{
    configuration::ConfigurationServerboundPackets, play::PlayServerboundPackets, V1_21_0,
};

use super::SettingsTrait;
use crate::{
    network::{config::ConfigPacketEvent, play::PlayClientPacketEvent},
    player::settings::ClientSettings,
};

impl SettingsTrait for V1_21_0 {
    fn config_settings(event: &ConfigPacketEvent<Self>) -> Option<ClientSettings> {
        if let ConfigurationServerboundPackets::ClientOptions(options) = event.packet.as_ref() {
            Some(ClientSettings {
                language: options.language.clone(),
                view_distance: options.view_distance,
                chat_visibility: options.chat_visibility,
                chat_colors: options.chat_colors,
                model_customization: options.model_customization,
                main_hand: options.main_hand,
                text_filtering_enabled: options.text_filtering_enabled,
                allows_listing: options.allows_listing,
            })
        } else {
            None
        }
    }

    fn play_settings(event: &PlayClientPacketEvent<Self>) -> Option<ClientSettings> {
        if let PlayServerboundPackets::ClientOptions(options) = event.packet.as_ref() {
            Some(ClientSettings {
                language: options.language.clone(),
                view_distance: options.view_distance,
                chat_visibility: options.chat_visibility,
                chat_colors: options.chat_colors,
                model_customization: options.model_customization,
                main_hand: options.main_hand,
                text_filtering_enabled: options.text_filtering_enabled,
                allows_listing: options.allows_listing,
            })
        } else {
            None
        }
    }
}
