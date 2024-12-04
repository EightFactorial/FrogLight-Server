//! TODO

use bevy::prelude::*;
use froglight::{network::connection::NetworkDirection, prelude::*};

#[doc(hidden)]
pub(super) fn build<V: Version>(_app: &mut App)
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
}
