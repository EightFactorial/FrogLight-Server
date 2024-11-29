use std::future::Future;

use froglight::{
    network::connection::NetworkDirection,
    prelude::{Version, *},
};

use super::LoginTask;
use crate::network::{channel, AsyncPacketChannel};

mod v1_21_0;

/// A trait that performs the login process.
pub trait LoginTrait: Version
where
    Clientbound: NetworkDirection<Self, Login>,
    Login: State<Self>,
{
    /// Create a new [`LoginTask`] with the given [`Connection`].
    #[must_use]
    fn new_login(conn: Connection<Self, Login, Clientbound>) -> LoginTask<Self> {
        let (packet, channel) = channel();
        LoginTask::spawn(packet, Self::login(conn, channel))
    }

    /// Complete the login process with the given [`Connection`].
    fn login(
        conn: Connection<Self, Login, Clientbound>,
        channel: AsyncPacketChannel<Self, Login>,
    ) -> impl Future<Output = Result<Connection<Self, Login, Clientbound>, ConnectionError>>
           + Send
           + 'static;
}
