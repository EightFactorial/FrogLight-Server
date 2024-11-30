//! TODO

use std::marker::PhantomData;

use bevy::{app::PluginGroupBuilder, prelude::*};
use froglight::prelude::Version;

pub mod action;
use action::{
    ConfigFinishPlugin, ConfigKnownPackPlugin, ConfigOptionsPlugin, ConfigRegistryPlugin,
    LoginProfilePlugin, PlayStartPlugin,
};

pub mod filter;

mod systemset;
pub use systemset::*;

static TARGET: &str = "NEXT";

/// A [`PluginGroup`] for the network extension plugins.
///
/// Contains:
/// - [`LoginProfilePlugin`] for sending login profiles.
/// - [`ConfigKnownPackPlugin`] for sending known resourcepacks.
/// - [`ConfigRegistryPlugin`] for sending registries.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct NetworkExtPlugins<V: Version>(PhantomData<V>);

impl<V: Version> PluginGroup for NetworkExtPlugins<V>
where
    LoginProfilePlugin<V>: Plugin,
    ConfigOptionsPlugin<V>: Plugin,
    ConfigKnownPackPlugin<V>: Plugin,
    ConfigRegistryPlugin<V>: Plugin,
    ConfigFinishPlugin<V>: Plugin,
    PlayStartPlugin<V>: Plugin,
{
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();

        // Add Login plugins
        builder = builder.add(LoginProfilePlugin::<V>::default());

        // Add Config plugins
        builder = builder.add(ConfigOptionsPlugin::<V>::default());
        builder = builder.add(ConfigKnownPackPlugin::<V>::default());
        builder = builder.add(ConfigRegistryPlugin::<V>::default());
        builder = builder.add(ConfigFinishPlugin::<V>::default());

        // Add Play plugins
        builder = builder.add(PlayStartPlugin::<V>::default());

        builder
    }
}
