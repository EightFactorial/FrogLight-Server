use std::future::Future;

use froglight::{
    network::connection::NetworkDirection,
    prelude::{
        Clientbound, Connection, ConnectionError, GameProfile, Login, Resolver, State, Version,
    },
};

use super::{AuthenticationServer, LoginTask};
use crate::network::common::AsyncPacketChannel;

mod v1_21_0;

///  A trait that defines the behavior of a login process.
pub trait LoginTrait: Version
where
    Clientbound: NetworkDirection<Self, Login>,
    Login: State<Self>,
{
    /// An async function that performs the login process.
    fn login(
        conn: Connection<Self, Login, Clientbound>,
        channel: AsyncPacketChannel<Self, Login>,
        auth_server: AuthenticationServer<Self>,
        resolver: Resolver,
    ) -> impl Future<Output = Result<Connection<Self, Login, Clientbound>, ConnectionError>> + Send + Sync;

    /// Send a [`GameProfile`] to the client.
    fn send_profile(profile: &GameProfile, task: &LoginTask<Self>);
}
