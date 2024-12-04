use std::sync::Arc;

use async_channel::{
    Receiver, Recv as RecvFut, Send as SendFut, Sender, TryRecvError, TrySendError,
};
use froglight::prelude::{State, *};

type SentPacket<V, S> = <S as State<V>>::ClientboundPacket;
type RecvPacket<V, S> = <S as State<V>>::ServerboundPacket;

/// Create a new [`PacketChannel`] and [`AsyncPacketChannel`] pair.
#[must_use]
pub fn channel<V: Version, S: State<V>>() -> (PacketChannel<V, S>, AsyncPacketChannel<V, S>) {
    let (p_send, p_recv) = async_channel::unbounded();
    let (a_send, a_recv) = async_channel::unbounded();
    (
        PacketChannel { send: p_send, recv: a_recv },
        AsyncPacketChannel { send: a_send, recv: p_recv },
    )
}

/// A channel for sending and receiving packets.
pub struct PacketChannel<V: Version, S: State<V>> {
    send: Sender<Arc<SentPacket<V, S>>>,
    recv: Receiver<Arc<RecvPacket<V, S>>>,
}

impl<V: Version, S: State<V>> PacketChannel<V, S> {
    /// Send a packet to the other side of the channel.
    pub fn send(&self, packet: impl Into<SentPacket<V, S>>) {
        let _ = self.try_send(Arc::new(packet.into()));
    }
    /// Send a packet to the other side of the channel.
    pub fn send_arc(&self, packet: Arc<SentPacket<V, S>>) { let _ = self.try_send(packet); }
    /// Try to send a packet to the other side of the channel.
    ///
    /// # Errors
    /// Returns an error if the channel has been closed.
    pub fn try_send(
        &self,
        packet: Arc<SentPacket<V, S>>,
    ) -> Result<(), TrySendError<Arc<SentPacket<V, S>>>> {
        self.send.try_send(packet)
    }

    /// Receive a packet from the other side of the channel.
    #[must_use]
    pub fn recv(&self) -> Option<Arc<RecvPacket<V, S>>> { self.try_recv().ok() }
    /// Try to receive a packet from the other side of the channel.
    ///
    /// # Errors
    /// Returns an error if the channel has been closed.
    pub fn try_recv(&self) -> Result<Arc<RecvPacket<V, S>>, TryRecvError> { self.recv.try_recv() }
}

/// An asynchronous version of [`PacketChannel`].
///
/// Used inside of an async task to send and receive packets.
pub struct AsyncPacketChannel<V: Version, S: State<V>> {
    send: Sender<Arc<RecvPacket<V, S>>>,
    recv: Receiver<Arc<SentPacket<V, S>>>,
}

impl<V: Version, S: State<V>> AsyncPacketChannel<V, S> {
    /// Send a packet to the other side of the channel.
    pub fn send(&self, packet: RecvPacket<V, S>) -> SendFut<'_, Arc<RecvPacket<V, S>>> {
        self.send.send(Arc::new(packet))
    }

    /// Receive a packet from the other side of the channel.
    pub fn recv(&self) -> RecvFut<'_, Arc<SentPacket<V, S>>> { self.recv.recv() }
}
