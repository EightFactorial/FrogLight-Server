//! Common network utilities and structures.
//!
//! These are completely independent of the protocol version and state,
//! and can be used in any network-related code.

mod channel;
pub use channel::{channel, AsyncPacketChannel, PacketChannel};

mod component;
pub use component::ComponentFilter;

mod event;
pub use event::{ConnectionStateEvent, PacketEvent};

mod filter;
pub use filter::{ConnectionFilter, FilterResult};

mod task;
pub use task::ConnectionTask;
