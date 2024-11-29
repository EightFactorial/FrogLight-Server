use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use bevy::log::trace;
use froglight::{
    network::versions::v1_21_0::{
        configuration::{ConfigurationClientboundPackets, ConfigurationServerboundPackets},
        V1_21_0,
    },
    prelude::*,
};

use crate::network::{config::TARGET, AsyncPacketChannel};

impl super::ConfigTrait for V1_21_0 {
    async fn config(
        conn: Connection<Self, Configuration, Clientbound>,
        channel: AsyncPacketChannel<Self, Configuration>,
    ) -> Result<Connection<Self, Configuration, Clientbound>, ConnectionError> {
        let (mut read, mut write) = conn.into_split();

        let finished = AtomicBool::default();
        let pending = AtomicU32::default();

        futures_lite::future::race(
            async {
                while !finished.load(Ordering::Relaxed) || pending.load(Ordering::Relaxed) > 0 {
                    if let Ok(packet) = channel.recv().await {
                        trace!(target: TARGET, "Sending config packet: {:?}", packet);

                        // Increment the pending counter if the packet requires a response
                        match packet.as_ref() {
                            ConfigurationClientboundPackets::CookieRequest(..)
                            | ConfigurationClientboundPackets::KeepAlive(..)
                            | ConfigurationClientboundPackets::CommonPing(..)
                            | ConfigurationClientboundPackets::ResourcePackSend(..)
                            | ConfigurationClientboundPackets::SelectKnownPacks(..)
                            | ConfigurationClientboundPackets::ServerLinks(..) => {
                                pending.fetch_add(1, Ordering::Relaxed);
                            }
                            _ => {}
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
                    trace!(target: TARGET, "Received config packet: {:?}", packet);

                    // Decrement the pending counter if the packet is a response
                    match &packet {
                        ConfigurationServerboundPackets::CookieResponse(..)
                        | ConfigurationServerboundPackets::KeepAlive(..)
                        | ConfigurationServerboundPackets::CommonPong(..)
                        | ConfigurationServerboundPackets::SelectKnownPacks(..) => {
                            pending.fetch_sub(1, Ordering::Relaxed);
                        }
                        // Decrement the pending counter if the packet
                        // is not an `Accepted` or `Declined` response
                        ConfigurationServerboundPackets::ResourcePackStatus(packet) => {
                            if !matches!(
                                packet.status,
                                ResourcePackStatus::Accepted | ResourcePackStatus::Declined
                            ) {
                                pending.fetch_sub(1, Ordering::Relaxed);
                            }
                        }
                        _ => {}
                    }

                    // If the client enters configuration, we can stop the loop
                    if let ConfigurationServerboundPackets::Ready(..) = &packet {
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
