//! TODO

use std::marker::PhantomData;

use bevy::{app::PluginGroupBuilder, prelude::*};
use froglight::prelude::Version;

pub mod keepalive;
use keepalive::KeepAlivePlugin;

/// A [`PluginGroup`] that adds player-related plugins to the [`App`].
#[derive(Debug, Default)]
pub struct PlayerPlugins<V: Version>(PhantomData<V>);

impl<V: Version> PluginGroup for PlayerPlugins<V>
where
    KeepAlivePlugin<V>: Plugin,
{
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();
        builder = builder.add(KeepAlivePlugin::<V>::default());
        builder
    }
}