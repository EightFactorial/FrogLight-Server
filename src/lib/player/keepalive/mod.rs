//! TODO

use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{network::connection::NetworkDirection, prelude::*};

use crate::network::{
    config::{ConfigPacketEvent, ConfigTask},
    login::LoginStateEvent,
    play::{PlayClientPacketEvent, PlayTask},
};

mod counter;
pub use counter::KeepAliveCounter;

mod version;
pub use version::KeepAliveTrait;

mod systemset;
pub use systemset::KeepAliveSystemSet;

/// A [`Plugin`] that sends and receives keep-alive packets.
#[derive(Debug, Default)]
pub struct KeepAlivePlugin<V: Version>(PhantomData<V>);

impl<V: Version + KeepAliveTrait> Plugin for KeepAlivePlugin<V>
where
    Clientbound:
        NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    fn build(&self, app: &mut App) {
        // Only configure `KeepAliveSystemSet` if it doesn't already exist.
        if !app
            .world()
            .resource::<Schedules>()
            .get(Update)
            .is_some_and(|s| s.graph().contains_set(KeepAliveSystemSet))
        {
            app.configure_sets(Update, KeepAliveSystemSet);
        }

        app.add_systems(
            Update,
            (
                KeepAliveCounter::add_keepalives::<V>
                    .run_if(on_event::<LoginStateEvent<V>>)
                    .ambiguous_with(KeepAliveCounter::tick_keepalives),
                KeepAliveCounter::tick_keepalives::<V>.run_if(
                    any_with_component::<PlayTask<V>>.or(any_with_component::<ConfigTask<V>>),
                ),
                KeepAliveCounter::receive_keepalives::<V>
                    .run_if(
                        on_event::<PlayClientPacketEvent<V>>.or(on_event::<ConfigPacketEvent<V>>),
                    )
                    .ambiguous_with(KeepAliveCounter::tick_keepalives),
            )
                .in_set(KeepAliveSystemSet),
        );
    }
}
