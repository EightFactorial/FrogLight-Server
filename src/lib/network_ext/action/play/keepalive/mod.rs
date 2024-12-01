use std::{
    marker::PhantomData,
    time::{Duration, Instant},
};

use bevy::prelude::*;
use froglight::{
    network::connection::NetworkDirection,
    prelude::{State, *},
};

use crate::{
    network::{PlayPacketEvent, PlayTask},
    network_ext::{NetworkExtPlaySet, TARGET},
};

mod v1_21_0;

/// A [`Plugin`] that manages client keepalive information.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayKeepAlivePlugin<V: Version>(PhantomData<V>);

/// A [`Component`] that stores a player's keepalive information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct ClientKeepAlive {
    /// The keepalive counter.
    pub keepalive: u64,
    /// The time the last keepalive packet was sent.
    pub sent: Instant,
    /// The time the last keepalive packet was received.
    pub received: Instant,
}

impl<V: Version + PlayKeepAliveTrait> Plugin for PlayKeepAlivePlugin<V>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (Self::insert_default_keepalive, Self::send_keepalive, Self::recv_keepalive)
                .in_set(NetworkExtPlaySet),
        );
    }
}

impl<V: Version + PlayKeepAliveTrait> PlayKeepAlivePlugin<V>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(10);
    const KEEPALIVE_THRESHOLD: Duration = Duration::from_secs(20);

    /// A system that inserts default keepalive information
    /// for connected clients.
    #[expect(clippy::type_complexity)]
    pub fn insert_default_keepalive(
        query: Query<(Entity, &GameProfile), (Added<PlayTask<V>>, Without<ClientKeepAlive>)>,
        mut commands: Commands,
    ) {
        for (entity, profile) in &query {
            debug!(target: TARGET, "Starting keepalives for {}", profile.name);
            commands.entity(entity).insert(ClientKeepAlive {
                keepalive: 0,
                sent: Instant::now(),
                received: Instant::now(),
            });
        }
    }

    /// A system that sends keepalive packets to connected clients.
    pub fn send_keepalive(
        mut query: Query<(Entity, &mut ClientKeepAlive, &GameProfile, &PlayTask<V>)>,
        mut commands: Commands,
    ) {
        for (entity, mut keepalive, profile, task) in &mut query {
            if keepalive.received.elapsed() >= Self::KEEPALIVE_THRESHOLD {
                // Kick the player if they haven't responded to the keepalive
                warn!(target: TARGET, "Disconnecting {} for not responding to keepalive", profile.name);
                commands.entity(entity).despawn();
            } else if keepalive.sent.elapsed() >= Self::KEEPALIVE_INTERVAL {
                // Increment the keepalive counter and send a new keepalive packet
                debug!(target: TARGET, "Sending keepalive to {}", profile.name);
                keepalive.sent = Instant::now();
                keepalive.keepalive += 1;
                V::send_keepalive(keepalive.keepalive, task);
            }
        }
    }

    const KEEPALIVE_HISTORY: u64 = 2;

    /// A system that receives keepalive packets from connected clients.
    pub fn recv_keepalive(
        mut query: Query<(&mut ClientKeepAlive, &GameProfile)>,
        mut events: EventReader<PlayPacketEvent<V>>,
        mut commands: Commands,
    ) {
        for event in events.read() {
            if let Some(received) = V::recv_keepalive(&event.packet) {
                if let Ok((mut keepalive, profile)) = query.get_mut(event.entity) {
                    if keepalive.keepalive.saturating_sub(received) > Self::KEEPALIVE_HISTORY {
                        // Kick the player if the keepalive is too old
                        warn!(target: TARGET, "Disconnecting {} for invalid keepalive", profile.name);
                        commands.entity(event.entity).despawn();
                    } else {
                        // Update the keepalive counter
                        debug!(target: TARGET, "Received keepalive from {}", profile.name);
                        keepalive.keepalive = received;
                        keepalive.received = Instant::now();
                    }
                }
            }
        }
    }
}

/// A trait that sends and receives keepalive packets
/// to and from connected clients.
pub trait PlayKeepAliveTrait: Version
where
    Clientbound: NetworkDirection<Self, Play>,
    Play: State<Self>,
{
    /// Sends a keepalive packet to a connected client.
    fn send_keepalive(keepalive: u64, task: &PlayTask<Self>);

    /// Receives a keepalive packet from a connected client.
    fn recv_keepalive(packet: &<Play as State<Self>>::ServerboundPacket) -> Option<u64>;
}
