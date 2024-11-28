use std::future::Future;

use bevy::{
    prelude::*,
    tasks::{block_on, poll_once, IoTaskPool, Task},
};
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use super::{AsyncConfigChannel, TaskConfigChannel};

mod v1_21_0;

/// A task that handles the Config process.
#[derive(Component, Deref)]
pub struct ConfigTask<V: Version>
where
    Clientbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    #[deref]
    channel: TaskConfigChannel<V>,
    task: Task<ConfigReturn<V>>,
}
type ConfigReturn<V> = Result<Connection<V, Play, Clientbound>, ConnectionError>;

impl<V: Version> ConfigTask<V>
where
    Clientbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    /// Poll the [`ConfigTask`].
    pub fn poll(&mut self) -> Option<ConfigReturn<V>> { block_on(poll_once(&mut self.task)) }
}

impl<V: Version + ConnectionConfig> ConfigTask<V>
where
    Clientbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    /// Spawn a [`ConfigTask`] for the given [`Connection`].
    #[must_use]
    pub fn spawn(connection: Connection<V, Configuration, Clientbound>) -> Self {
        let (task_channel, async_channel) = super::channel();
        let task =
            IoTaskPool::get().spawn(<V as ConnectionConfig>::configure(connection, async_channel));
        Self { channel: task_channel, task }
    }
}

pub trait ConnectionConfig: Version
where
    Clientbound: NetworkDirection<Self, Configuration> + NetworkDirection<Self, Play>,
    Configuration: State<Self>,
    Play: State<Self>,
{
    fn configure(
        _connection: Connection<Self, Configuration, Clientbound>,
        _channel: AsyncConfigChannel<Self>,
    ) -> impl Future<Output = ConfigReturn<Self>> + Send + Sync;
}
