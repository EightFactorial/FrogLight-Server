use froglight::{network::connection::NetworkDirection, prelude::*};

use super::KeepAliveCounter;
use crate::network::{
    config::{ConfigPacketEvent, ConfigTask},
    play::{PlayClientPacketEvent, PlayTask},
};

mod v1_21_0;

/// A trait that provides keep-alive functionality.
pub trait KeepAliveTrait: Version
where
    Clientbound: NetworkDirection<Self, Configuration> + NetworkDirection<Self, Play>,
    Configuration: State<Self>,
    Play: State<Self>,
{
    /// Send a keep-alive packet during the configuration phase.
    fn send_config(keepalive: &mut KeepAliveCounter, task: &ConfigTask<Self>);
    /// Send a keep-alive packet during the play session.
    fn send_play(keepalive: &mut KeepAliveCounter, task: &PlayTask<Self>);

    /// Receive a keep-alive packet during the configuration phase.
    fn recv_config(
        keepalive: &mut KeepAliveCounter,
        event: &ConfigPacketEvent<Self>,
    ) -> Option<bool>;
    /// Receive a keep-alive packet during the play session.
    fn recv_play(
        keepalive: &mut KeepAliveCounter,
        event: &PlayClientPacketEvent<Self>,
    ) -> Option<bool>;
}
