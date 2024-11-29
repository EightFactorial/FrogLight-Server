use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use bevy::log::trace;
use froglight::{
    network::versions::v1_21_0::{
        login::{LoginClientboundPackets, LoginServerboundPackets},
        V1_21_0,
    },
    prelude::*,
};

use crate::network::{login::TARGET, AsyncPacketChannel};

impl super::LoginTrait for V1_21_0 {
    async fn login(
        conn: Connection<Self, Login, Clientbound>,
        channel: AsyncPacketChannel<Self, Login>,
    ) -> Result<Connection<Self, Login, Clientbound>, ConnectionError> {
        let (mut read, mut write) = conn.into_split();

        let finished = AtomicBool::default();
        let pending = AtomicU32::default();

        futures_lite::future::race(
            async {
                while !finished.load(Ordering::Relaxed) || pending.load(Ordering::Relaxed) > 0 {
                    if let Ok(packet) = channel.recv().await {
                        trace!(target: TARGET, "Sending login packet: {:?}", packet);

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
                        return Ok::<(), ConnectionError>(());
                    }
                }
                Ok::<(), ConnectionError>(())
            },
            async {
                while !finished.load(Ordering::Relaxed) || pending.load(Ordering::Relaxed) > 0 {
                    let packet = read.recv().await?;
                    trace!(target: TARGET, "Received login packet: {:?}", packet);

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
                        return Ok::<(), ConnectionError>(());
                    }
                }
                Ok::<(), ConnectionError>(())
            },
        )
        .await?;

        Ok(Connection::from_split(read, write).await)
    }
}
