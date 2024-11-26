use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    core::TaskPoolPlugin,
};
use froglight::{
    network::{NetworkPlugin as FroglightNetworkPlugin, ResolverPlugin as FroglightResolverPlugin},
    HeadlessPlugins,
};

use crate::plugin::NetworkPlugin;

/// A [`PluginGroup`] for creating a server.
///
/// Contains all the plugins required to run a server.
///
/// FrogLight's [`HeadlessPlugins`]:
/// - [`PanicHandlerPlugin`](bevy::app::PanicHandlerPlugin)
/// - [`LogPlugin`](bevy::log::LogPlugin)
/// - [`TaskPoolPlugin`]
/// - [`TypeRegistrationPlugin`](bevy::core::TypeRegistrationPlugin)
/// - [`FrameCountPlugin`](bevy::core::FrameCountPlugin)
/// - [`TimePlugin`](bevy::time::TimePlugin)
/// - [`TransformPlugin`](bevy::transform::TransformPlugin)
/// - [`HierarchyPlugin`](bevy::hierarchy::HierarchyPlugin)
/// - [`DiagnosticsPlugin`](bevy::diagnostic::DiagnosticsPlugin)
/// - [`ScheduleRunnerPlugin`](bevy::app::ScheduleRunnerPlugin)
/// - [`TerminalCtrlCHandlerPlugin`](bevy::app::TerminalCtrlCHandlerPlugin)
/// - [`StatesPlugin`](bevy::state::app::StatesPlugin)
/// - [`BlockPlugin`](froglight::prelude::plugins::BlockPlugin)
/// - [`UtilityPlugin`](froglight::prelude::plugins::UtilityPlugin)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ServerPlugins {
    /// The address the server will bind to.
    pub socket: SocketAddr,
}

impl ServerPlugins {
    /// The default socket address for the server.
    pub const LOCALHOST: SocketAddr =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 25565);
}
impl Default for ServerPlugins {
    fn default() -> Self { Self { socket: Self::LOCALHOST } }
}

impl PluginGroup for ServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();
        builder = builder.add_group(HeadlessPlugins);

        // Disable the network and resolver plugins.
        builder = builder.disable::<FroglightNetworkPlugin>().disable::<FroglightResolverPlugin>();
        // Use the custom network plugin.
        builder = builder.add(NetworkPlugin { socket: self.socket });

        // Overwrite the default TaskPoolPlugin settings.
        builder = builder.set(TaskPoolPlugin { task_pool_options: super::TASKPOOL_SETTINGS });

        builder
    }
}
