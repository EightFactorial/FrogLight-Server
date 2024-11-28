//! Plugins used in creating the server.

pub mod filter;
pub use filter::{AcceptedConnectionEvent, ConnectionFilter, ConnectionFilterPlugin};

mod group;
pub use group::{ServerPlugins, TASKPOOL_SETTINGS};

pub mod listen;
pub use listen::{ConnectionListener, ListenerPlugin};

pub mod login;
pub use login::{ConnectionLoginEvent, LoginAction, LoginChecklist, LoginPacketEvent, LoginPlugin};

pub mod configuration;
pub use configuration::{
    ConfigAction, ConfigChecklist, ConfigPacketEvent, ConfigPlugin, ConnectionConfigEvent,
};
