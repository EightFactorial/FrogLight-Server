//! TODO
use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{network::connection::NetworkDirection, prelude::*};
use initialize::PlayerInitialize;

pub mod entity;
pub mod initialize;
pub mod keepalive;
pub mod positioner;
pub mod superflat;

/// TODO
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorldPlugin<V: Version>(PhantomData<V>);

impl<V: Version + PlayerInitialize> Plugin for WorldPlugin<V>
where
    Clientbound: NetworkDirection<V, Login> + NetworkDirection<V, Play>,
    Login: State<V>,
    Play: State<V>,
{
    fn build(&self, app: &mut App) {
        keepalive::build::<V>(app);
        positioner::build::<V>(app);
        initialize::build::<V>(app);

        entity::build(app);
        superflat::build(app);
    }
}
