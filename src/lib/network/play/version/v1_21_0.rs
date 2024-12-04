use std::sync::atomic::{AtomicBool, Ordering};

use froglight::{
    network::versions::v1_21_0::{
        play::{EnterReconfigurationPacket, PlayServerboundPackets},
        V1_21_0,
    },
    prelude::*,
};

use super::PlayTrait;
use crate::network::{common::AsyncPacketChannel, play::PlayTask};

impl PlayTrait for V1_21_0 {
    async fn play(
        conn: Connection<Self, Play, Clientbound>,
        channel: AsyncPacketChannel<Self, Play>,
    ) -> Result<Connection<Self, Play, Clientbound>, ConnectionError> {
        let (mut read, mut write) = conn.into_split();

        let finished = AtomicBool::default();

        futures_lite::future::or(
            async {
                while !finished.load(Ordering::Relaxed) {
                    if let Ok(packet) = channel.recv().await {
                        #[cfg(debug_assertions)]
                        bevy::log::trace!("Sending play packet: {packet:?}");
                        write.send_packet(&packet).await?;
                    } else {
                        break;
                    }
                }
                Ok::<(), ConnectionError>(())
            },
            async {
                while !finished.load(Ordering::Relaxed) {
                    let packet = read.recv().await?;
                    #[cfg(debug_assertions)]
                    bevy::log::trace!("Received config packet: {packet:?}");

                    // If the client enters reconfiguration, we can stop the loop
                    if let PlayServerboundPackets::AcknowledgeReconfiguration(..) = &packet {
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

    fn send_reconfigure(task: &PlayTask<Self>) { task.send(EnterReconfigurationPacket); }
}
