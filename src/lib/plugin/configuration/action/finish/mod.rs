use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use super::{ClientBrand, ClientKnownPacks};
use crate::configuration::ConfigTask;

mod v1_21_0;

/// A component that marks the end of the configuration process.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfigFinish;

impl ConfigFinish {
    /// Finish the configuration process for the given [`ConfigTask`]s.
    #[expect(private_bounds)]
    pub fn finish_configuration<V: Version + FinishConfig>(
        query: Query<&ConfigTask<V>, (With<ClientBrand>, With<ClientKnownPacks>)>,
    ) where
        Clientbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
        Configuration: State<V>,
        Play: State<V>,
    {
        for task in &query {
            V::send_finish(task);
        }
    }
}

pub(crate) trait FinishConfig: Version
where
    Clientbound: NetworkDirection<Self, Configuration> + NetworkDirection<Self, Play>,
    Configuration: State<Self>,
    Play: State<Self>,
{
    fn send_finish(task: &ConfigTask<Self>);
}
