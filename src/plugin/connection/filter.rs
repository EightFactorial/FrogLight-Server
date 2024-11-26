use std::net::IpAddr;

use bevy::{
    prelude::{Resource, World},
    utils::{hashbrown::Equivalent, HashSet},
};
use compact_str::CompactString;
use derive_more::derive::From;
use froglight::prelude::Uuid;

use super::FilterResult;
use crate::plugin::listen::ConnectionRequest;

/// A set of UUIDs, usernames, and addresses to filter connections by.
#[derive(Debug, Default, Clone, PartialEq, Eq, Resource)]
pub struct ConnectionFilter {
    /// A set of filtered UUIDs, usernames, and addresses.
    pub filters: HashSet<FilterEntry>,
    /// The filter mode.
    pub mode: FilterMode,
}

/// An entry in a [`ConnectionFilter`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, From)]
pub enum FilterEntry {
    /// A player's UUID.
    Uuid(Uuid),
    /// A player's username.
    Username(CompactString),
    /// An IP address.
    Address(IpAddr),
}

/// The mode of a [`ConnectionFilter`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilterMode {
    /// Only allow connections that are in the list.
    AllowList,
    /// Deny connections that are in the list.
    #[default]
    DenyList,
}

impl ConnectionFilter {
    /// The reason for denying a connection from the `AllowList`.
    const ALLOWLIST_REASON: &str = "Connection filtered by AllowList";
    /// The reason for denying a connection from the `DenyList`.
    const DENYLIST_REASON: &str = "Connection filtered by DenyList";

    /// Add a [`FilterEntry`] to the list.
    pub fn add(&mut self, entry: impl Into<FilterEntry>) { self.filters.insert(entry.into()); }

    /// Add a [`FilterEntry::Username`] to the list.
    pub fn add_username(&mut self, username: impl Into<CompactString>) {
        self.add(FilterEntry::Username(username.into()));
    }

    /// Add a [`FilterEntry::Uuid`] to the list.
    pub fn add_uuid(&mut self, uuid: impl Into<Uuid>) { self.add(FilterEntry::Uuid(uuid.into())); }

    /// Add a [`FilterEntry::Address`] to the list.
    pub fn add_address(&mut self, address: impl Into<IpAddr>) {
        self.add(FilterEntry::Address(address.into()));
    }

    /// Remove a [`FilterEntry`] from the list.
    pub fn remove<'a>(&mut self, entry: impl Into<FilterRef<'a>>) -> bool {
        self.filters.remove(&entry.into())
    }

    /// A [`FilterFn`](super::FilterFn) that
    /// uses [`FilterEntry`]s to filter connections.
    pub fn filter(request: &ConnectionRequest, world: &World) -> FilterResult {
        if let Some(Self { filters, mode }) = world.get_resource::<Self>() {
            // Create filter references for the request fields
            let filter_uuid = FilterRef::Uuid(&request.uuid);
            let filter_username = FilterRef::Username(&request.username);

            let address = request.socket.ip();
            let filter_address = FilterRef::Address(&address);

            // Check if the request matches any filters
            let contains = filters.contains(&filter_uuid)
                || filters.contains(&filter_username)
                || filters.contains(&filter_address);

            // Match based on the filter mode
            match (contains, mode) {
                // Allow if the request is in the AllowList or not in the DenyList
                (true, FilterMode::AllowList) | (false, FilterMode::DenyList) => {
                    FilterResult::Allow
                }
                // Deny if the request is in the DenyList
                (true, FilterMode::DenyList) => {
                    FilterResult::Deny(Some(Self::ALLOWLIST_REASON.into()))
                }
                // Deny if the request is not in the AllowList
                (false, FilterMode::AllowList) => {
                    FilterResult::Deny(Some(Self::DENYLIST_REASON.into()))
                }
            }
        } else {
            FilterResult::Allow
        }
    }
}

/// A reference used to compare to a [`FilterEntry`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From)]
pub enum FilterRef<'a> {
    Uuid(&'a Uuid),
    Username(&'a CompactString),
    Address(&'a IpAddr),
}
impl Equivalent<FilterEntry> for FilterRef<'_> {
    fn equivalent(&self, key: &FilterEntry) -> bool {
        match (*self, key) {
            (Self::Uuid(uuid), FilterEntry::Uuid(other)) => uuid == other,
            (Self::Username(username), FilterEntry::Username(other)) => username == other,
            (Self::Address(address), FilterEntry::Address(other)) => address == other,
            _ => false,
        }
    }
}

impl From<&str> for FilterEntry {
    fn from(username: &str) -> Self { Self::Username(username.into()) }
}
