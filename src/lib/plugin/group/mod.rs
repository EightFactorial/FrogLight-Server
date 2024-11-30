mod taskpool;
use std::net::SocketAddr;

use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    core::TaskPoolPlugin,
    log::LogPlugin,
};
use froglight::{
    network::{
        versions::v1_21_0::V1_21_0, NetworkPlugin as FroglightNetworkPlugin,
        ResolverPlugin as FroglightResolverPlugin,
    },
    DefaultPlugins,
};
pub use taskpool::TASKPOOL_SETTINGS;

use crate::{
    network::{NetworkPlugins, SocketPlugin},
    network_ext::NetworkExtPlugins,
    world::WorldPlugins,
};

/// A [`PluginGroup`] for creating a server.
///
/// Contains all the plugins required to run a server.
///
/// Bevy's [`DefaultPlugins`](bevy::DefaultPlugins):
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
/// Froglight's [`DefaultPlugins`]:
/// - [`BlockPlugin`](froglight::prelude::plugins::BlockPlugin)
/// - [`EntityPlugin`](froglight::prelude::plugins::EntityPlugin)
/// - [`RegistryPlugin`](froglight::prelude::plugins::RegistryPlugin)
/// - [`UtilityPlugin`](froglight::prelude::plugins::UtilityPlugin)
///
/// FrogLight-Server's plugins:
/// - [`SocketPlugin`](crate::network::SocketPlugin)
/// - [`LoginPlugin`](crate::network::LoginPlugin)
/// - [`ConfigPlugin`](crate::network::ConfigPlugin)
/// - [`PlayPlugin`](crate::network::PlayPlugin)
/// - [`LoginProfilePlugin`](crate::network_ext::action::LoginProfilePlugin)
/// - [`ConfigKnownPackPlugin`](crate::network_ext::action::ConfigKnownPackPlugin)
/// - [`ConfigRegistryPlugin`](crate::network_ext::action::ConfigRegistryPlugin)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ServerPlugins {
    /// The address the server will bind to.
    pub socket: SocketAddr,
}

impl ServerPlugins {
    #[cfg(debug_assertions)]
    const LOG_FILTER: &'static str = "info,CONF=debug,LOGN=debug,NEXT=debug,PLAY=debug,SOCK=debug";

    #[cfg(not(debug_assertions))]
    const LOG_FILTER: &'static str = "info";
}
impl Default for ServerPlugins {
    fn default() -> Self { Self { socket: SocketPlugin::<V1_21_0>::LOCALHOST } }
}

impl PluginGroup for ServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();
        builder = builder.add_group(DefaultPlugins);

        // Configure the `LogPlugin`.
        builder = builder
            .set(LogPlugin { filter: String::from(Self::LOG_FILTER), ..LogPlugin::default() });

        // Overwrite the default `TaskPoolPlugin` settings.
        builder = builder.set(TaskPoolPlugin { task_pool_options: super::TASKPOOL_SETTINGS });

        // Disable the `FroglightNetworkPlugin` and `FroglightResolverPlugin`.
        builder = builder.disable::<FroglightNetworkPlugin>().disable::<FroglightResolverPlugin>();

        // Add the v1.21.0 `NetworkPlugins` and `NetworkExtPlugins`.
        builder = builder
            .add_group(NetworkPlugins::<V1_21_0>::from_socket(self.socket))
            .add_group(NetworkExtPlugins::<V1_21_0>::default());

        // Add the `WorldPlugins`.
        builder = builder.add_group(WorldPlugins);

        builder
    }
}
