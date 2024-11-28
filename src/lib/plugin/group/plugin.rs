use std::net::SocketAddr;

use bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    core::TaskPoolPlugin,
    log::LogPlugin,
};
use froglight::{
    network::{NetworkPlugin as FroglightNetworkPlugin, ResolverPlugin as FroglightResolverPlugin},
    HeadlessPlugins,
};

use crate::plugin::{ConfigPlugin, ConnectionFilterPlugin, ListenerPlugin, LoginPlugin};

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
/// - [`ListenerPlugin`]
/// - [`ConnectionFilterPlugin`]
/// - [`LoginPlugin`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ServerPlugins {
    /// The address the server will bind to.
    pub socket: SocketAddr,
}

impl ServerPlugins {
    #[cfg(debug_assertions)]
    const LOG_FILTER: &'static str = "info,CONFG=debug,FILTR=debug,NETWK=debug";

    #[cfg(not(debug_assertions))]
    const LOG_FILTER: &'static str = "info";
}
impl Default for ServerPlugins {
    fn default() -> Self { Self { socket: ListenerPlugin::LOCALHOST } }
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

        // Disable the NetworkPlugin and ResolverPlugin.
        builder = builder.disable::<FroglightNetworkPlugin>().disable::<FroglightResolverPlugin>();
        // Add the ListenerPlugin, ConnectionFilterPlugin,
        // LoginPlugin, and ConfigPlugin.
        builder = builder.add(ListenerPlugin { socket: self.socket });
        builder = builder.add(ConnectionFilterPlugin).add(LoginPlugin).add(ConfigPlugin);

        builder
    }
}
