use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use bevy::log::trace;
use froglight::{
    network::versions::v1_21_0::{
        configuration::{
            ConfigurationClientboundPackets, ConfigurationServerboundPackets, ReadyS2CPacket,
        },
        V1_21_0,
    },
    prelude::*,
};

use super::ConfigTrait;
use crate::network::{common::AsyncPacketChannel, config::ConfigTask};

impl ConfigTrait for V1_21_0 {
    async fn config(
        conn: Connection<Self, Configuration, Clientbound>,
        channel: AsyncPacketChannel<Self, Configuration>,
    ) -> Result<Connection<Self, Configuration, Clientbound>, ConnectionError> {
        let (mut read, mut write) = conn.into_split();

        let finished = AtomicBool::default();
        let pending = AtomicU32::default();

        futures_lite::future::or(
            async {
                while !finished.load(Ordering::Relaxed) || pending.load(Ordering::Relaxed) > 0 {
                    if let Ok(packet) = channel.recv().await {
                        #[cfg(debug_assertions)]
                        trace!(
                            "Sending config packet: {packet:?}, pending: {}",
                            pending.load(Ordering::Relaxed)
                        );

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
                        break;
                    }
                }
                Ok::<(), ConnectionError>(())
            },
            async {
                while !finished.load(Ordering::Relaxed) || pending.load(Ordering::Relaxed) > 0 {
                    let packet = read.recv().await?;
                    #[cfg(debug_assertions)]
                    trace!(
                        "Received config packet: {packet:?}, pending: {}",
                        pending.load(Ordering::Relaxed)
                    );

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
                        break;
                    }
                }
                Ok::<(), ConnectionError>(())
            },
        )
        .await?;

        Ok(Connection::from_split(read, write).await)
    }

    fn send_finish(task: &ConfigTask<Self>) { task.send(ReadyS2CPacket); }
}
