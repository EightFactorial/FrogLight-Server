use std::{net::SocketAddr, time::Duration};

use async_channel::{Receiver, Sender, TryRecvError};
use async_std::net::TcpListener;
use bevy::{
    prelude::*,
    tasks::{block_on, poll_once, IoTaskPool, Task},
};
use compact_str::CompactString;
use froglight::{
    network::versions::v1_21_0::{
        handshake::HandshakeServerboundPackets,
        login::LoginServerboundPackets,
        play::PingResultPacket,
        status::{QueryResponsePacket, StatusServerboundPackets},
        V1_21_0,
    },
    prelude::*,
};

use super::ServerStatusArc;
use crate::plugin::listen::TARGET;

/// A listener for incoming connections.
#[derive(Resource)]
pub struct ConnectionListener {
    recv: Receiver<ConnectionRequest>,
    task: Task<()>,
}

impl ConnectionListener {
    /// Receive a pending [`ConnectionRequest`], if there is one.
    #[must_use]
    pub fn recv(&self) -> Option<ConnectionRequest> { self.try_recv().ok() }
    /// Try to receive a pending [`ConnectionRequest`].
    ///
    /// # Errors
    /// Returns an error if there are no pending requests
    /// or if the channel is closed.
    pub fn try_recv(&self) -> Result<ConnectionRequest, TryRecvError> { self.recv.try_recv() }

    /// Poll the listener task.
    pub fn poll(&mut self) -> Option<()> { block_on(poll_once(&mut self.task)) }
}

/// An incoming connection request.
pub struct ConnectionRequest {
    /// The username of the client.
    pub username: CompactString,
    /// The UUID of the client.
    pub uuid: Uuid,
    /// The protocol version of the client.
    pub protocol: i32,
    /// The server the client is connecting to.
    pub server: CompactString,
    /// The intent of the connection.
    pub intent: ConnectionIntent,
    /// The socket address of the client.
    pub socket: SocketAddr,
    /// The connection to the client.
    pub connection: Connection<V1_21_0, Login, Clientbound>,
}

impl ConnectionListener {
    /// How long to wait for a connection to complete before timing out.
    const CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);

    /// Create a new [`ConnectionListener`] that listens on the given socket.
    ///
    /// # Errors
    /// Errors if the listener fails to bind to the socket.
    pub fn new(socket: SocketAddr, status: ServerStatusArc) -> Result<Self, std::io::Error> {
        info!(target: TARGET, "Listening at {socket}");

        let (send, recv) = async_channel::unbounded();
        let listener = block_on(async move { TcpListener::bind(socket).await })?;
        let task = IoTaskPool::get().spawn(Self::listen(listener, status, send));

        Ok(Self { recv, task })
    }

    /// Listen for incoming connections.
    async fn listen(
        listener: TcpListener,
        status: ServerStatusArc,
        channel: Sender<ConnectionRequest>,
    ) {
        let taskpool = IoTaskPool::get();
        while let Ok((stream, socket)) = listener.accept().await {
            trace!(target: TARGET, "{socket} : Accepted connection");

            // Create a connection from the stream.
            let connection = match Connection::from_async_stream(stream) {
                Ok(conn) => conn,
                Err(err) => {
                    error!(target: TARGET, "{socket} : {err}");
                    continue;
                }
            };

            let status = status.clone();
            let channel = channel.clone();

            // Spawn a task to handle the incoming connection.
            // Timeout after `CONNECTION_TIMEOUT` seconds.
            taskpool
                .spawn(async move {
                    if async_std::future::timeout(
                        Self::CONNECTION_TIMEOUT,
                        Self::handle_incoming(connection, socket, status, channel),
                    )
                    .await
                    .is_err()
                    {
                        error!(target: TARGET, "{socket} : Connection timed out");
                    }
                })
                .detach();
        }
    }

    /// Handle an incoming connection.
    async fn handle_incoming(
        mut connection: Connection<V1_21_0, Handshake, Clientbound>,
        socket: SocketAddr,
        status: ServerStatusArc,
        channel: Sender<ConnectionRequest>,
    ) {
        let Ok(HandshakeServerboundPackets::Handshake(handshake)) = connection.recv().await else {
            error!(target: TARGET, "{socket} : Failed to receive handshake packet");
            return;
        };

        match handshake.intent {
            // Send the connection request to the main thread.
            ConnectionIntent::Login | ConnectionIntent::Transfer => {
                debug!(target: TARGET, "{socket} : Login");
                let mut connection = connection.login();

                // Receive the hello packet.
                let hello = match connection.recv().await {
                    Ok(LoginServerboundPackets::LoginHello(hello)) => hello,
                    Ok(_) => {
                        error!(target: TARGET, "{socket} : Failed to receive hello packet");
                        return;
                    }
                    Err(err) => {
                        error!(target: TARGET, "{socket} : {err}");
                        return;
                    }
                };

                // Send the connection request to the main thread.
                if channel
                    .send(ConnectionRequest {
                        username: hello.username,
                        uuid: hello.uuid,
                        protocol: handshake.protocol,
                        server: handshake.address,
                        intent: handshake.intent,
                        socket,
                        connection,
                    })
                    .await
                    .is_err()
                {
                    error!(target: TARGET, "Failed to send connection request to main thread");
                };
            }
            // Handle the status request.
            ConnectionIntent::Status => {
                debug!(target: TARGET, "{socket} : Status");

                let mut connection = connection.status();
                let mut counter = 0u32;

                loop {
                    match connection.recv().await {
                        Ok(StatusServerboundPackets::QueryRequest(..)) => {
                            trace!(target: TARGET, "{socket} : Status Request");
                            let response = QueryResponsePacket { status: status.read().clone() };
                            if let Err(err) = connection.send(response).await {
                                error!(target: TARGET, "{socket} : {err}");
                            }
                        }
                        Ok(StatusServerboundPackets::QueryPing(query)) => {
                            trace!(target: TARGET, "{socket} : Ping Request");
                            let response = PingResultPacket { pong: query.ping };
                            if let Err(err) = connection.send(response).await {
                                error!(target: TARGET, "{socket} : {err}");
                            }

                            // Close the connection after sending the response.
                            return;
                        }
                        Err(err) => {
                            error!(target: TARGET, "{socket} : Failed to receive status packet: {err}");
                        }
                    }

                    // Limit the number of packets to prevent spam.
                    counter += 1;
                    if counter >= 3 {
                        return;
                    }
                }
            }
        }
    }
}
