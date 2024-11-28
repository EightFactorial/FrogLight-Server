use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::prelude::*;

mod types;
pub use types::*;

/// A plugin for managing [`Connection`]s in the [`Configuration`] state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfigPlugin<V: Version> {
    _phantom: PhantomData<V>,
}

impl<V: Version> Plugin for ConfigPlugin<V> {
    fn build(&self, _app: &mut App) {}
}
