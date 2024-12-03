//! The core [`ServerPlugins`] [`PluginGroup`] for creating a server.
//!
//! Includes a custom taskpool configuration,
//! [`TASKPOOL_SETTINGS`], for bevy's [`TaskPoolPlugin`].

use std::{net::SocketAddr, str::FromStr};

use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    core::TaskPoolPlugin,
    log::LogPlugin,
    DefaultPlugins,
};

mod taskpool;
pub use taskpool::TASKPOOL_SETTINGS;

use crate::dimension::DimensionPlugin;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ServerPlugins {
    /// The address the server will bind to.
    pub socket: SocketAddr,
}

impl ServerPlugins {
    #[cfg(debug_assertions)]
    const LOG_FILTER: &'static str = "info,froglight_server=debug";

    #[cfg(not(debug_assertions))]
    const LOG_FILTER: &'static str = "info";
}
impl Default for ServerPlugins {
    fn default() -> Self { Self { socket: SocketAddr::from_str("127.0.0.1:25565").unwrap() } }
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

        // Add the v1.21.0 `NetworkPlugins` and `NetworkExtPlugins`.
        // builder = builder
        //     .add_group(NetworkPlugins::<V1_21_0>::from_socket(self.socket))
        //     .add_group(NetworkExtPlugins::<V1_21_0>::default());

        // Add the `RegistryPlugins` and `WorldPlugins`.
        // builder = builder.add_group(RegistryPlugins).add_group(WorldPlugins);

        builder
    }
}
