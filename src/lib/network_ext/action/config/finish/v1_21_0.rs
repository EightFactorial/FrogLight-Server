use froglight::network::versions::v1_21_0::{configuration::ReadyS2CPacket, V1_21_0};

use super::ConfigFinishTrait;
use crate::network::ConfigTask;

impl ConfigFinishTrait for V1_21_0 {
    fn send_finish(task: &ConfigTask<Self>) { task.send(ReadyS2CPacket); }
}
