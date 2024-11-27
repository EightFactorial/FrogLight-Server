use std::future::Future;

use bevy::{
    prelude::*,
    tasks::{block_on, poll_once, IoTaskPool, Task},
};
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use super::{AsyncLoginChannel, TaskLoginChannel};

mod v1_21_0;

/// A task that handles the login process.
#[derive(Component, Deref)]
pub struct LoginTask<V: Version>
where
    Clientbound: NetworkDirection<V, Login> + NetworkDirection<V, Configuration>,
    Login: State<V>,
    Configuration: State<V>,
{
    #[deref]
    channel: TaskLoginChannel<V>,
    task: Task<LoginReturn<V>>,
}
type LoginReturn<V> = Result<Connection<V, Configuration, Clientbound>, ConnectionError>;

impl<V: Version> LoginTask<V>
where
    Clientbound: NetworkDirection<V, Login> + NetworkDirection<V, Configuration>,
    Login: State<V>,
    Configuration: State<V>,
{
    /// Poll the [`LoginTask`].
    pub fn poll(&mut self) -> Option<LoginReturn<V>> { block_on(poll_once(&mut self.task)) }
}

impl<V: Version + ConnectionLogin> LoginTask<V>
where
    Clientbound: NetworkDirection<V, Login> + NetworkDirection<V, Configuration>,
    Login: State<V>,
    Configuration: State<V>,
{
    /// Spawn a [`LoginTask`] for the given [`Connection`].
    #[must_use]
    pub fn spawn(connection: Connection<V, Login, Clientbound>) -> Self {
        let (task_channel, async_channel) = super::channel();
        let task =
            IoTaskPool::get().spawn(<V as ConnectionLogin>::login(connection, async_channel));
        Self { channel: task_channel, task }
    }
}

pub trait ConnectionLogin: Version
where
    Clientbound: NetworkDirection<Self, Login> + NetworkDirection<Self, Configuration>,
    Login: State<Self>,
    Configuration: State<Self>,
{
    fn login(
        _connection: Connection<Self, Login, Clientbound>,
        _channel: AsyncLoginChannel<Self>,
    ) -> impl Future<Output = LoginReturn<Self>> + Send + Sync;
}
