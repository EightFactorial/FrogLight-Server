use std::future::Future;

use bevy::{
    prelude::*,
    tasks::{block_on, poll_once, IoTaskPool, Task},
};
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use super::PacketChannel;

type TaskResult<V, S> = Result<Connection<V, S, Clientbound>, ConnectionError>;

/// A task that represents a connection to a client.
#[derive(Component, Deref)]
pub struct ConnectionTask<V: Version, S: State<V>>
where
    Clientbound: NetworkDirection<V, S>,
{
    #[deref]
    channel: PacketChannel<V, S>,
    task: Task<TaskResult<V, S>>,
}

impl<V: Version, S: State<V>> ConnectionTask<V, S>
where
    Clientbound: NetworkDirection<V, S>,
{
    /// Spawn a new [`ConnectionTask`] with the given
    /// [`PacketChannel`] and future.
    pub fn spawn(
        channel: PacketChannel<V, S>,
        future: impl Future<Output = TaskResult<V, S>> + Send + 'static,
    ) -> Self {
        Self { channel, task: IoTaskPool::get().spawn(future) }
    }

    /// Poll the [`ConnectionTask`] once.
    ///
    /// # Note
    /// This will panic if the task returns `Some` and is then polled again.
    pub fn poll(&mut self) -> Option<TaskResult<V, S>> { block_on(poll_once(&mut self.task)) }
}
