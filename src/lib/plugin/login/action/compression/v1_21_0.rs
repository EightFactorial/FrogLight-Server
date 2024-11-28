use froglight::network::versions::v1_21_0::{login::LoginCompressionPacket, V1_21_0};

use crate::login::LoginTask;

impl super::SendCompression for V1_21_0 {
    fn send_compression(threshold: u32, task: &LoginTask<Self>) {
        task.send(LoginCompressionPacket { threshold });
    }
}
