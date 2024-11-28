use std::sync::Arc;

use async_channel::{
    Receiver, Recv as RecvFut, Send as SendFut, Sender, TryRecvError, TrySendError,
};
use froglight::prelude::*;

/// Create a new [`TaskConfigChannel`] and [`AsyncConfigChannel`] pair.
#[must_use]
pub fn channel<V: Version>() -> (TaskConfigChannel<V>, AsyncConfigChannel<V>)
where
    Configuration: State<V>,
{
    let (task_send, task_recv) = async_channel::unbounded();
    let (async_send, async_recv) = async_channel::unbounded();
    (
        TaskConfigChannel { send: task_send, recv: async_recv },
        AsyncConfigChannel { send: async_send, recv: task_recv },
    )
}

/// A channel for sending and receiving Config packets.
pub struct TaskConfigChannel<V: Version>
where
    Configuration: State<V>,
{
    send: Sender<Arc<<Configuration as State<V>>::ClientboundPacket>>,
    recv: Receiver<Arc<<Configuration as State<V>>::ServerboundPacket>>,
}

impl<V: Version> TaskConfigChannel<V>
where
    Configuration: State<V>,
{
    /// Send a packet to the channel.
    pub fn send(&self, packet: impl Into<<Configuration as State<V>>::ClientboundPacket>) {
        let _ = self.try_send(Arc::new(packet.into()));
    }
    /// Send a packet to the channel.
    ///
    /// # Errors
    /// Returns an error if the channel is full or closed.
    pub fn try_send(
        &self,
        packet: Arc<<Configuration as State<V>>::ClientboundPacket>,
    ) -> Result<(), TrySendError<Arc<<Configuration as State<V>>::ClientboundPacket>>> {
        self.send.try_send(packet)
    }

    /// Receive a packet from the channel.
    #[must_use]
    pub fn recv(&self) -> Option<Arc<<Configuration as State<V>>::ServerboundPacket>> {
        self.try_recv().ok()
    }
    /// Receive a packet from the channel.
    ///
    /// # Errors
    /// Returns an error if the channel is empty or closed.
    pub fn try_recv(
        &self,
    ) -> Result<Arc<<Configuration as State<V>>::ServerboundPacket>, TryRecvError> {
        self.recv.try_recv()
    }
}

/// A channel for sending and receiving Config packets asynchronously.
pub struct AsyncConfigChannel<V: Version>
where
    Configuration: State<V>,
{
    send: Sender<Arc<<Configuration as State<V>>::ServerboundPacket>>,
    recv: Receiver<Arc<<Configuration as State<V>>::ClientboundPacket>>,
}

impl<V: Version> AsyncConfigChannel<V>
where
    Configuration: State<V>,
{
    /// Send a packet to the channel.
    pub fn send(
        &self,
        packet: <Configuration as State<V>>::ServerboundPacket,
    ) -> SendFut<'_, Arc<<Configuration as State<V>>::ServerboundPacket>> {
        self.send.send(Arc::new(packet))
    }

    /// Receive a packet from the channel.
    pub fn recv(&self) -> RecvFut<'_, Arc<<Configuration as State<V>>::ClientboundPacket>> {
        self.recv.recv()
    }
}
