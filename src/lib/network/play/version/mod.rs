use std::future::Future;

use froglight::{
    network::connection::NetworkDirection,
    prelude::{Version, *},
};

use super::PlayTask;
use crate::network::{channel, AsyncPacketChannel};

mod v1_21_0;

/// A trait that manages the [`Play`] process.
pub trait PlayTrait: Version
where
    Clientbound: NetworkDirection<Self, Play>,
    Play: State<Self>,
{
    /// Create a new [`PlayTask`] with the given [`Connection`].
    #[must_use]
    fn new_play(conn: Connection<Self, Play, Clientbound>) -> PlayTask<Self> {
        let (packet, channel) = channel();
        PlayTask::spawn(packet, Self::play(conn, channel))
    }

    /// Complete the play process with the given [`Connection`].
    fn play(
        conn: Connection<Self, Play, Clientbound>,
        channel: AsyncPacketChannel<Self, Play>,
    ) -> impl Future<Output = Result<Connection<Self, Play, Clientbound>, ConnectionError>>
           + Send
           + 'static;
}
