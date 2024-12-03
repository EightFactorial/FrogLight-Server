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
use froglight::network::versions::v1_21_0::V1_21_0;
pub use taskpool::TASKPOOL_SETTINGS;

use crate::{network::SocketPlugin, DimensionPlugin, NetworkPlugins};

/// A [`PluginGroup`] for creating a server.
///
/// Contains all the plugins required to run a server.
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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ServerPlugins {
    /// The address the server will bind to.
    pub socket: Option<SocketAddr>,
}

impl ServerPlugins {
    #[cfg(debug_assertions)]
    const LOG_FILTER: &'static str = "info,froglight_server=debug";

    #[cfg(not(debug_assertions))]
    const LOG_FILTER: &'static str = "info";

    /// Create a new [`ServerPlugins`].
    ///
    /// The server will not bind to any address.
    #[must_use]
    pub const fn new() -> Self { Self { socket: None } }

    /// Create a new [`ServerPlugins`] that listens on `127.0.0.1`.
    #[must_use]
    pub const fn localhost() -> Self { Self::from_socket(SocketPlugin::<V1_21_0>::LOCALHOST) }

    /// Create a new [`ServerPlugins`] that listens on `0.0.0.0`.
    #[must_use]
    pub const fn public() -> Self { Self::from_socket(SocketPlugin::<V1_21_0>::PUBLIC) }

    /// Create a new [`ServerPlugins`] that listens on the given socket.
    #[must_use]
    pub const fn from_socket(socket: SocketAddr) -> Self { Self { socket: Some(socket) } }

    /// Set the [`SocketAddr`] for the server.
    ///
    /// This will listen on the given socket for incoming connections.
    #[must_use]
    pub const fn with_socket(mut self, socket: SocketAddr) -> Self {
        self.socket = Some(socket);
        self
    }
}

impl PluginGroup for ServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();
        builder = builder.add_group(DefaultPlugins);

        // Configure the `LogPlugin`.
        builder = builder
            .set(LogPlugin { filter: String::from(Self::LOG_FILTER), ..LogPlugin::default() });

        // Configure the `TaskPoolPlugin` settings.
        builder = builder.set(TaskPoolPlugin { task_pool_options: TASKPOOL_SETTINGS });

        // Add the Dimension plugin.
        builder = builder.add(DimensionPlugin);

        // Add the v1.21.0 `NetworkPlugins`.
        builder = builder.add_group(NetworkPlugins::<V1_21_0>::from_option(self.socket));

        builder
    }
}
