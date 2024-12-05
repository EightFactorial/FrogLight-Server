//! The core [`ServerPlugins`] [`PluginGroup`] for creating a server.
//!
//! Includes a custom taskpool configuration,
//! [`TASKPOOL_SETTINGS`], for bevy's [`TaskPoolPlugin`].

use std::net::SocketAddr;

use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    core::TaskPoolPlugin,
    log::LogPlugin,
    DefaultPlugins,
};

mod taskpool;
use compact_str::CompactString;
use froglight::{
    network::{versions::v1_21_0::V1_21_0, ResolverPlugin},
    prelude::plugins::EntityPlugin,
    registry::RegistryPlugin,
};
pub use taskpool::TASKPOOL_SETTINGS;

use crate::{
    network::{LoginPlugin, SocketPlugin},
    DimensionPlugin, NetworkPlugins, PlayerPlugins,
};

/// A [`PluginGroup`] for creating a server.
///
/// Contains all the plugins required to run a server.
///
/// Currently uses: [`V1_21_0`].
///
/// Bevy's [`DefaultPlugins`]:
/// - [`PanicHandlerPlugin`](bevy::app::PanicHandlerPlugin)
/// - [`LogPlugin`](bevy::log::LogPlugin)
/// - [`TaskPoolPlugin`](bevy::core::TaskPoolPlugin)
/// - [`TypeRegistrationPlugin`](bevy::core::TypeRegistrationPlugin)
/// - [`FrameCountPlugin`](bevy::core::FrameCountPlugin)
/// - [`TimePlugin`](bevy::time::TimePlugin)
/// - [`TransformPlugin`](bevy::transform::TransformPlugin)
/// - [`HierarchyPlugin`](bevy::hierarchy::HierarchyPlugin)
/// - [`DiagnosticsPlugin`](bevy::diagnostic::DiagnosticsPlugin)
/// - [`InputPlugin`](bevy::input::InputPlugin)
/// - [`ScheduleRunnerPlugin`](bevy::app::ScheduleRunnerPlugin)
/// - [`WindowPlugin`](bevy::window::WindowPlugin)
/// - [`AccessibilityPlugin`](bevy::a11y::AccessibilityPlugin)
/// - [`TerminalCtrlCHandlerPlugin`](bevy::app::TerminalCtrlCHandlerPlugin)
/// - [`AssetPlugin`](bevy::asset::AssetPlugin)
/// - [`ScenePlugin`](bevy::scene::ScenePlugin)
/// - [`WinitPlugin`](bevy::winit::WinitPlugin)
/// - [`RenderPlugin`](bevy::render::RenderPlugin)
/// - [`ImagePlugin`](bevy::render::texture::ImagePlugin)
/// - [`PipelinedRenderingPlugin`](bevy::render::pipelined_rendering::PipelinedRenderingPlugin)
/// - [`CorePipelinePlugin`](bevy::core_pipeline::CorePipelinePlugin)
/// - [`SpritePlugin`](bevy::sprite::SpritePlugin)
/// - [`TextPlugin`](bevy::text::TextPlugin)
/// - [`UiPlugin`](bevy::ui::UiPlugin)
/// - [`PbrPlugin`](bevy::pbr::PbrPlugin)
/// - [`GltfPlugin`](bevy::gltf::GltfPlugin)
/// - [`AudioPlugin`](bevy::audio::AudioPlugin)
/// - [`GilrsPlugin`](bevy::gilrs::GilrsPlugin)
/// - [`AnimationPlugin`](bevy::animation::AnimationPlugin)
/// - [`GizmoPlugin`](bevy::gizmos::GizmoPlugin)
/// - [`StatesPlugin`](bevy::state::app::StatesPlugin)
/// - [`DevToolsPlugin`](bevy::dev_tools::DevToolsPlugin)
/// - [`DefaultPickingPlugins`](bevy::picking::DefaultPickingPlugins)
///
/// FrogLight-Server's plugins:
/// - [`DimensionPlugin`]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServerPlugins {
    /// The address the server will bind to.
    pub socket: Option<SocketAddr>,
    /// The address of the authentication server.
    pub auth_server: Option<CompactString>,
}

impl ServerPlugins {
    #[cfg(debug_assertions)]
    const LOG_FILTER: &'static str = "info,froglight_server=debug";

    #[cfg(not(debug_assertions))]
    const LOG_FILTER: &'static str = "info";

    /// Create a new [`ServerPlugins`] that listens on `127.0.0.1`.
    #[must_use]
    pub const fn localhost() -> Self { Self::from_socket(SocketPlugin::<V1_21_0>::LOCALHOST) }

    /// Create a new [`ServerPlugins`] that listens on `0.0.0.0`.
    #[must_use]
    pub const fn public() -> Self { Self::from_socket(SocketPlugin::<V1_21_0>::PUBLIC) }

    /// Create a new [`ServerPlugins`] that listens on the given socket.
    #[must_use]
    pub const fn from_socket(socket: SocketAddr) -> Self { Self::from_option(Some(socket)) }

    /// Create a new [`ServerPlugins`] that listens on the given socket,
    /// if it is [`Some`].
    #[must_use]
    pub const fn from_option(socket: Option<SocketAddr>) -> Self {
        Self { socket, auth_server: None }
    }

    /// Set the [`SocketAddr`] for the server.
    ///
    /// This will listen on the given socket for incoming connections.
    #[must_use]
    pub const fn with_socket(mut self, socket: SocketAddr) -> Self {
        self.socket = Some(socket);
        self
    }

    /// Set the [`SocketAddr`] for the server, if it is [`None`].
    #[must_use]
    pub const fn or_with_socket(self, socket: SocketAddr) -> Self {
        if self.socket.is_none() {
            self.with_socket(socket)
        } else {
            self
        }
    }

    /// Remove the authentication server.
    #[must_use]
    pub fn offline(mut self) -> Self {
        self.auth_server = None;
        self
    }

    /// Use the Mojang authentication server.
    #[must_use]
    pub fn online(self) -> Self { self.with_auth(LoginPlugin::<V1_21_0>::MOJANG_SERVER) }

    /// Set the address of the authentication server.
    #[must_use]
    pub fn with_auth(mut self, auth_server: CompactString) -> Self {
        self.auth_server = Some(auth_server);
        self
    }

    /// Set the address of the authentication server, if it is [`None`].
    #[must_use]
    pub fn or_with_auth(self, auth_server: CompactString) -> Self {
        if self.auth_server.is_none() {
            self.with_auth(auth_server)
        } else {
            self
        }
    }
}

impl Default for ServerPlugins {
    fn default() -> Self { Self::localhost().offline() }
}

impl PluginGroup for ServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();
        builder = builder.add_group(DefaultPlugins);

        // Add FrogLight's `EntityPlugin`, `RegistryPlugin`, and `ResolverPlugin`.
        builder = builder.add(EntityPlugin).add(RegistryPlugin).add(ResolverPlugin::default());

        // Configure the `LogPlugin`.
        builder = builder
            .set(LogPlugin { filter: String::from(Self::LOG_FILTER), ..LogPlugin::default() });

        // Configure the `TaskPoolPlugin` settings.
        builder = builder.set(TaskPoolPlugin { task_pool_options: TASKPOOL_SETTINGS });

        // Add the `Dimension` plugin.
        builder = builder.add(DimensionPlugin);

        // Add the v1.21.0 `NetworkPlugins`.
        builder = builder.add_group(NetworkPlugins::<V1_21_0>::from_option(self.socket));
        // Add the v1.21.0 `PlayerPlugins`.
        builder = builder.add_group(PlayerPlugins::<V1_21_0>::default());

        builder
    }
}
