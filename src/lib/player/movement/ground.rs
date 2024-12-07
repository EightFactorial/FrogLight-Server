use bevy::prelude::*;
use derive_more::{From, Into};

/// A [`Component`] that returns whether the client reports being on the ground.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, Component, Reflect,
)]
#[reflect(Default, Component)]
pub struct ClientGrounded(pub bool);

impl Default for ClientGrounded {
    fn default() -> Self { Self(true) }
}
