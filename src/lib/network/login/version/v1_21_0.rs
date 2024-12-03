use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use bevy::log::trace;
use froglight::{
    network::versions::v1_21_0::{
        login::{LoginClientboundPackets, LoginServerboundPackets, LoginSuccessPacket},
        V1_21_0,
    },
    prelude::*,
};

use super::LoginTrait;
use crate::network::{
    common::AsyncPacketChannel,
    login::{AuthenticationServer, LoginTask},
};

impl LoginTrait for V1_21_0 {
    async fn login(
        conn: Connection<Self, Login, Clientbound>,
        channel: AsyncPacketChannel<Self, Login>,
        _auth_server: AuthenticationServer<Self>,
        _resolver: Resolver,
    ) -> Result<Connection<Self, Login, Clientbound>, ConnectionError> {
        let (mut read, mut write) = conn.into_split();

        let finished = AtomicBool::default();
        let pending = AtomicU32::default();

        futures_lite::future::or(
            async {
                while !finished.load(Ordering::Relaxed) || pending.load(Ordering::Relaxed) > 0 {
                    if let Ok(packet) = channel.recv().await {
                        trace!("Sending login packet: {packet:?}");

                        // Increment the pending counter if the packet is a request
                        if matches!(
                            packet.as_ref(),
                            LoginClientboundPackets::CookieRequest(..)
                                | LoginClientboundPackets::LoginQueryRequest(..)
                        ) {
                            pending.fetch_add(1, Ordering::Relaxed);
                        }

                        write.send_packet(&packet).await?;
                    } else {
                        break;
                    }
                }
                Ok::<(), ConnectionError>(())
            },
            async {
                while !finished.load(Ordering::Relaxed) || pending.load(Ordering::Relaxed) > 0 {
                    let packet = read.recv().await?;
                    trace!("Received login packet: {packet:?}");

                    // Decrement the pending counter if the packet is a response
                    if matches!(
                        packet,
                        LoginServerboundPackets::CookieResponse(..)
                            | LoginServerboundPackets::LoginQueryResponse(..)
                    ) {
                        pending.fetch_sub(1, Ordering::Relaxed);
                    }

                    // If the client enters configuration, we can stop the loop
                    if let LoginServerboundPackets::EnterConfiguration(..) = &packet {
                        finished.store(true, Ordering::Relaxed);
                    }

                    if channel.send(packet).await.is_err() {
                        break;
                    }
                }
                Ok::<(), ConnectionError>(())
            },
        )
        .await?;

        Ok(Connection::from_split(read, write).await)
    }

    fn send_profile(profile: &GameProfile, task: &LoginTask<Self>) {
        task.send(LoginSuccessPacket { profile: profile.clone(), strict_error_handling: false });
    }
}
