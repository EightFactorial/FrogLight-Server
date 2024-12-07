use froglight::network::versions::v1_21_0::V1_21_0;

use super::ConfigRegistryTrait;
use crate::network::config::ConfigTask;

impl ConfigRegistryTrait for V1_21_0 {
    fn send_registries(_task: &ConfigTask<Self>) {}
}
