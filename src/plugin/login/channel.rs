use std::sync::Arc;

use async_channel::{
    Receiver, Recv as RecvFut, Send as SendFut, Sender, TryRecvError, TrySendError,
};
use froglight::prelude::*;

/// Create a new [`TaskLoginChannel`] and [`AsyncLoginChannel`] pair.
#[must_use]
pub fn channel<V: Version>() -> (TaskLoginChannel<V>, AsyncLoginChannel<V>)
where
    Login: State<V>,
{
    let (task_send, task_recv) = async_channel::unbounded();
    let (async_send, async_recv) = async_channel::unbounded();
    (
        TaskLoginChannel { send: task_send, recv: async_recv },
        AsyncLoginChannel { send: async_send, recv: task_recv },
    )
}

/// A channel for sending and receiving login packets.
pub struct TaskLoginChannel<V: Version>
where
    Login: State<V>,
{
    send: Sender<Arc<<Login as State<V>>::ClientboundPacket>>,
    recv: Receiver<Arc<<Login as State<V>>::ServerboundPacket>>,
}

impl<V: Version> TaskLoginChannel<V>
where
    Login: State<V>,
{
    /// Send a packet to the channel.
    pub fn send(&self, packet: <Login as State<V>>::ClientboundPacket) {
        let _ = self.try_send(Arc::new(packet));
    }
    /// Send a packet to the channel.
    ///
    /// # Errors
    /// Returns an error if the channel is full or closed.
    pub fn try_send(
        &self,
        packet: Arc<<Login as State<V>>::ClientboundPacket>,
    ) -> Result<(), TrySendError<Arc<<Login as State<V>>::ClientboundPacket>>> {
        self.send.try_send(packet)
    }

    /// Receive a packet from the channel.
    #[must_use]
    pub fn recv(&self) -> Option<Arc<<Login as State<V>>::ServerboundPacket>> {
        self.try_recv().ok()
    }
    /// Receive a packet from the channel.
    ///
    /// # Errors
    /// Returns an error if the channel is empty or closed.
    pub fn try_recv(&self) -> Result<Arc<<Login as State<V>>::ServerboundPacket>, TryRecvError> {
        self.recv.try_recv()
    }
}

/// A channel for sending and receiving login packets asynchronously.
pub struct AsyncLoginChannel<V: Version>
where
    Login: State<V>,
{
    send: Sender<Arc<<Login as State<V>>::ServerboundPacket>>,
    recv: Receiver<Arc<<Login as State<V>>::ClientboundPacket>>,
}

impl<V: Version> AsyncLoginChannel<V>
where
    Login: State<V>,
{
    /// Send a packet to the channel.
    pub fn send(
        &self,
        packet: <Login as State<V>>::ServerboundPacket,
    ) -> SendFut<'_, Arc<<Login as State<V>>::ServerboundPacket>> {
        self.send.send(Arc::new(packet))
    }

    /// Receive a packet from the channel.
    pub fn recv(&self) -> RecvFut<'_, Arc<<Login as State<V>>::ClientboundPacket>> {
        self.recv.recv()
    }
}
