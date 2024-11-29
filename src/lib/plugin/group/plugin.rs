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
    HeadlessPlugins,
};

use crate::network::{NetworkPlugins, SocketPlugin};

/// A [`PluginGroup`] for creating a server.
///
/// Contains all the plugins required to run a server.
///
/// FrogLight's [`HeadlessPlugins`]:
/// - [`PanicHandlerPlugin`](bevy::app::PanicHandlerPlugin)
/// - [`LogPlugin`]
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
///
/// FrogLight-Server plugins:
/// - [`NetworkPlugins`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ServerPlugins {
    /// The address the server will bind to.
    pub socket: SocketAddr,
}

impl ServerPlugins {
    #[cfg(debug_assertions)]
    const LOG_FILTER: &'static str = "info,LOGN=debug,SOCK=debug";

    #[cfg(not(debug_assertions))]
    const LOG_FILTER: &'static str = "info";
}
impl Default for ServerPlugins {
    fn default() -> Self { Self { socket: SocketPlugin::<V1_21_0>::LOCALHOST } }
}

impl PluginGroup for ServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();
        builder = builder.add_group(HeadlessPlugins);

        // Configure the LogPlugin.
        builder = builder
            .set(LogPlugin { filter: String::from(Self::LOG_FILTER), ..LogPlugin::default() });

        // Overwrite the default TaskPoolPlugin settings.
        builder = builder.set(TaskPoolPlugin { task_pool_options: super::TASKPOOL_SETTINGS });

        // Disable the FroglightNetworkPlugin and FroglightResolverPlugin.
        builder = builder.disable::<FroglightNetworkPlugin>().disable::<FroglightResolverPlugin>();

        // Add the generic NetworkPlugins groups.
        builder = builder.add_group(NetworkPlugins::<V1_21_0>::from_socket(self.socket));

        builder
    }
}
