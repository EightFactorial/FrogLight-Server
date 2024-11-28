use froglight::network::versions::v1_21_0::{configuration::ReadyS2CPacket, V1_21_0};

use crate::configuration::ConfigTask;

impl super::FinishConfig for V1_21_0 {
    fn send_finish(task: &ConfigTask<Self>) { task.send(ReadyS2CPacket); }
}
