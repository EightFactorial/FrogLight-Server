use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use bevy::log::trace;
use froglight::{
    network::versions::v1_21_0::{
        configuration::{ConfigurationClientboundPackets, ConfigurationServerboundPackets},
        V1_21_0,
    },
    prelude::*,
};

use super::{AsyncConfigChannel, ConfigReturn};
use crate::configuration::TARGET;

impl super::ConnectionConfig for V1_21_0 {
    async fn configure(
        connection: Connection<Self, Configuration, Clientbound>,
        channel: AsyncConfigChannel<Self>,
    ) -> ConfigReturn<Self> {
        let (mut read, mut write) = connection.into_split();

        let finished = AtomicBool::default();
        let pending = AtomicU32::default();

        futures_lite::future::race(
            async {
                while !finished.load(Ordering::Relaxed) || pending.load(Ordering::Relaxed) > 0 {
                    if let Ok(packet) = channel.recv().await {
                        trace!(target: TARGET, "Sending config packet: {:?}", packet);

                        // Increment the pending counter if the packet requires a response
                        if matches!(
                            packet.as_ref(),
                            ConfigurationClientboundPackets::CookieRequest(..)
                                | ConfigurationClientboundPackets::CustomPayload(..)
                                | ConfigurationClientboundPackets::KeepAlive(..)
                                | ConfigurationClientboundPackets::CommonPing(..)
                                | ConfigurationClientboundPackets::ResourcePackSend(..)
                                | ConfigurationClientboundPackets::SelectKnownPacks(..)
                                | ConfigurationClientboundPackets::ServerLinks(..)
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
                    trace!(target: TARGET, "Received config packet: {:?}", packet);

                    // Decrement the pending counter if the packet is a response
                    match &packet {
                        ConfigurationServerboundPackets::CookieResponse(..)
                        | ConfigurationServerboundPackets::CustomPayload(..)
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

        Ok(Connection::from_split(read, write).await.play())
    }
}
