use froglight::{network::connection::NetworkDirection, prelude::*};

use super::ConfigTask;

mod v1_21_0;

///  A trait that defines sending registry values to clients.
pub trait ConfigRegistryTrait: Version
where
    Clientbound: NetworkDirection<Self, Configuration>,
    Configuration: State<Self>,
{
    /// Send registries to the client.
    fn send_registries(task: &ConfigTask<Self>);
}
