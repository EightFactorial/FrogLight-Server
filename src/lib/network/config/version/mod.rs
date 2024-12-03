use std::future::Future;

use froglight::{
    network::connection::NetworkDirection,
    prelude::{Clientbound, Configuration, Connection, ConnectionError, State, Version},
};

use super::ConfigTask;
use crate::network::common::AsyncPacketChannel;

mod v1_21_0;

///  A trait that defines the behavior of the configuration process.
pub trait ConfigTrait: Version
where
    Clientbound: NetworkDirection<Self, Configuration>,
    Configuration: State<Self>,
{
    /// An async function that performs the configuration process.
    fn config(
        conn: Connection<Self, Configuration, Clientbound>,
        channel: AsyncPacketChannel<Self, Configuration>,
    ) -> impl Future<Output = Result<Connection<Self, Configuration, Clientbound>, ConnectionError>>
           + Send
           + Sync;

    /// Send a finish packet to the client.
    fn send_finish(task: &ConfigTask<Self>);
}
