use std::sync::atomic::{AtomicBool, Ordering};

use froglight::{
    network::versions::v1_21_0::{login::LoginServerboundPackets, V1_21_0},
    prelude::*,
};

use super::{AsyncLoginChannel, LoginReturn};

impl super::ConnectionLogin for V1_21_0 {
    async fn login(
        connection: Connection<Self, Login, Clientbound>,
        channel: AsyncLoginChannel<Self>,
    ) -> LoginReturn<Self> {
        let (mut read, mut write) = connection.into_split();
        let finished = AtomicBool::default();

        futures_lite::future::race(
            async {
                while !finished.load(Ordering::Relaxed) {
                    if let Ok(packet) = channel.recv().await {
                        write.send_packet(&packet).await?;
                    } else {
                        return Ok::<(), ConnectionError>(());
                    }
                }
                Ok::<(), ConnectionError>(())
            },
            async {
                while !finished.load(Ordering::Relaxed) {
                    let packet = read.recv().await?;

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

        Ok(Connection::from_split(read, write).await.configuration())
    }
}
