//! TODO

use std::marker::PhantomData;

use bevy::{app::PluginGroupBuilder, prelude::*};
use froglight::prelude::Version;

pub mod action;
use action::ConnectionLoginPlugin;

pub mod filter;

mod systemset;
pub use systemset::*;

static TARGET: &str = "NEXT";

/// A [`PluginGroup`] for the network extension plugins.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct NetworkExtPlugins<V: Version>(PhantomData<V>);

impl<V: Version> PluginGroup for NetworkExtPlugins<V>
where
    ConnectionLoginPlugin<V>: Plugin,
{
    fn build(self) -> PluginGroupBuilder {
        let builder = PluginGroupBuilder::start::<Self>();
        builder.add(ConnectionLoginPlugin::default())
    }
}
