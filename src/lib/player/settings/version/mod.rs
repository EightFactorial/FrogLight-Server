use froglight::{network::connection::NetworkDirection, prelude::*};

use super::ClientSettings;
use crate::network::{config::ConfigPacketEvent, play::PlayClientPacketEvent};

mod v1_21_0;

/// A trait that receives client settings packets.
pub trait SettingsTrait: Version
where
    Clientbound: NetworkDirection<Self, Configuration> + NetworkDirection<Self, Play>,
    Configuration: State<Self>,
    Play: State<Self>,
{
    /// Receive the client's config settings packet.
    fn config_settings(event: &ConfigPacketEvent<Self>) -> Option<ClientSettings>;

    /// Receive the client's play settings packet.
    fn play_settings(event: &PlayClientPacketEvent<Self>) -> Option<ClientSettings>;
}
