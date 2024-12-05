//! TODO
#![feature(build_hasher_default_const_new)]
#![feature(const_type_id)]

pub mod dimension;
pub use dimension::DimensionPlugin;

pub mod network;
pub use network::NetworkPlugins;

pub mod player;
pub use player::PlayerPlugins;

pub mod plugin;
pub use plugin::ServerPlugins;
