use std::future::Future;

use froglight::{network::connection::NetworkDirection, prelude::*};

use super::PlayTask;
use crate::network::common::AsyncPacketChannel;

mod v1_21_0;

///  A trait that defines the behavior of playing clients.
pub trait PlayTrait: Version
where
    Clientbound: NetworkDirection<Self, Play>,
    Play: State<Self>,
{
    /// An async function that performs the playing process.
    fn play(
        conn: Connection<Self, Play, Clientbound>,
        channel: AsyncPacketChannel<Self, Play>,
    ) -> impl Future<Output = Result<Connection<Self, Play, Clientbound>, ConnectionError>> + Send + Sync;

    /// Send a reconfigure packet to the client.
    fn send_reconfigure(task: &PlayTask<Self>);
}
