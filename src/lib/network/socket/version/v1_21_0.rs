use std::{net::SocketAddr, sync::Arc};

use async_channel::Sender;
use async_std::{future::timeout, net::TcpListener};
use bevy::{
    log::{debug, error, trace, warn},
    tasks::IoTaskPool,
    utils::HashMap,
};
use froglight::{
    network::{
        connection::ConnectionInformation,
        versions::v1_21_0::{
            handshake::HandshakeServerboundPackets,
            login::LoginServerboundPackets,
            play::PingResultPacket,
            status::{QueryResponsePacket, StatusServerboundPackets},
            V1_21_0,
        },
    },
    prelude::*,
};
use parking_lot::{Mutex, RwLock};

use super::SocketTrait;
use crate::network::socket::ConnectionRequest;

impl SocketTrait for V1_21_0 {
    fn status() -> ServerStatus {
        ServerStatus {
            description: "A Froglight server".into(),
            favicon: None,
            players: ServerPlayers { max: 20, online: 0, sample: Vec::new() },
            version: ServerVersion { name: "1.21.1".into(), protocol: V1_21_0::ID },
            enforces_secure_chat: Some(false),
            other: HashMap::new(),
        }
    }

    async fn listen(
        listener: TcpListener,
        status: Arc<RwLock<ServerStatus>>,
        channel: Sender<ConnectionRequest<Self>>,
    ) {
        let taskpool = IoTaskPool::get();
        while let Ok((stream, sock)) = listener.accept().await {
            trace!("Incoming connection from {sock}");

            // Create a connection from the stream.
            let conn = match Connection::from_async_stream(stream) {
                Ok(conn) => conn,
                Err(error) => {
                    error!("Failed to create connection from {sock}: {error}");
                    continue;
                }
            };

            // Spawn a task and detach it.
            let channel = channel.clone();
            let status = status.clone();

            let task = taskpool.spawn(async move {
                if timeout(Self::TIMEOUT, handle(conn, sock, status, channel)).await.is_err() {
                    error!("Connection from {sock} timed out");
                }
            });
            task.detach();
        }
    }
}

async fn handle(
    mut conn: Connection<V1_21_0, Handshake, Clientbound>,
    socket: SocketAddr,
    status: Arc<RwLock<ServerStatus>>,
    channel: Sender<ConnectionRequest<V1_21_0>>,
) {
    let Ok(HandshakeServerboundPackets::Handshake(handshake)) = conn.recv().await else {
        error!("Failed to receive handshake from {socket}");
        return;
    };

    match handshake.intent {
        ConnectionIntent::Login | ConnectionIntent::Transfer => {
            debug!("Received login intent from {socket}");

            // Receive the login hello packet.
            let mut conn = conn.login();
            let Ok(LoginServerboundPackets::LoginHello(hello)) = conn.recv().await else {
                error!("Failed to receive login hello from {socket}");
                return;
            };

            // Send the request to the main thread.
            if let Err(err) = channel
                .send(ConnectionRequest {
                    username: hello.username,
                    uuid: hello.uuid,
                    information: ConnectionInformation { address: Some(handshake.address), socket },
                    connection: Mutex::new(Some(conn)),
                })
                .await
            {
                error!("Failed to send connection request to task: {err}");
            }
        }
        ConnectionIntent::Status => {
            debug!("Received status intent from {socket}");

            let mut conn = conn.status();
            let mut counter = 0;

            loop {
                match conn.recv().await {
                    // Send a query response.
                    Ok(StatusServerboundPackets::QueryRequest(..)) => {
                        trace!("Received status request from {socket}");

                        let status = status.read().clone();
                        if let Err(err) = conn.send(QueryResponsePacket { status }).await {
                            error!("Failed to send status response to {socket}: {err}");
                            return;
                        }
                    }
                    // Send a ping response, then close the connection.
                    Ok(StatusServerboundPackets::QueryPing(request)) => {
                        trace!("Received ping request from {socket}");

                        if let Err(err) = conn.send(PingResultPacket { pong: request.ping }).await {
                            error!("Failed to send pong to {socket}: {err}");
                        }

                        return;
                    }
                    // Close the connection.
                    Err(error) => {
                        error!("Failed to receive status packet from {socket}: {error}");
                        return;
                    }
                }

                // Limit the amount of packets that are processed to prevent abuse.
                counter += 1;
                if counter >= V1_21_0::MAX_PACKETS {
                    warn!("Too many status packets from {socket}");
                    return;
                }
            }
        }
    }
}
