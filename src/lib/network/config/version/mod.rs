use std::future::Future;

use froglight::{
    network::connection::NetworkDirection,
    prelude::{Version, *},
};

use super::ConfigTask;
use crate::network::{channel, AsyncPacketChannel};

mod v1_21_0;

/// A trait that performs the configuration process.
pub trait ConfigTrait: Version
where
    Clientbound: NetworkDirection<Self, Configuration>,
    Configuration: State<Self>,
{
    /// Create a new [`ConfigTask`] with the given [`Connection`].
    #[must_use]
    fn new_config(conn: Connection<Self, Configuration, Clientbound>) -> ConfigTask<Self> {
        let (packet, channel) = channel();
        ConfigTask::spawn(packet, Self::config(conn, channel))
    }

    /// Complete the configuration process with the given [`Connection`].
    fn config(
        conn: Connection<Self, Configuration, Clientbound>,
        channel: AsyncPacketChannel<Self, Configuration>,
    ) -> impl Future<Output = Result<Connection<Self, Configuration, Clientbound>, ConnectionError>>
           + Send
           + 'static;
}
