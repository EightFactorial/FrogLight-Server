//! TODO
#![feature(build_hasher_default_const_new)]
#![feature(const_type_id)]

pub mod dimension;
pub use dimension::DimensionPlugin;

pub mod network;
pub use network::NetworkPlugins;

pub mod plugin;
pub use plugin::ServerPlugins;

pub mod world;
pub use world::WorldPlugin;
