use std::time::Duration;

use bevy::prelude::*;
use froglight::{network::connection::NetworkDirection, prelude::*};

use super::KeepAliveTrait;
use crate::network::{
    config::{ConfigPacketEvent, ConfigTask},
    login::LoginStateEvent,
    play::{PlayClientPacketEvent, PlayTask},
};

/// A [`Component`] that keeps track of keep-alive packets.
#[derive(Debug, Component)]
pub struct KeepAliveCounter {
    current: u64,
    send: Timer,
    recv: Timer,
}
impl Default for KeepAliveCounter {
    fn default() -> Self { Self::new() }
}

impl KeepAliveCounter {
    /// How much the keep-alive counter can
    /// vary before it is considered invalid.
    pub const VALUE_THRESHOLD: u64 = 1;

    /// The interval at which keep-alive packets are sent.
    pub const PACKET_INTERVAL: f32 = 10f32;

    /// The maximum amount of time that can
    /// pass before a connection is considered lost.
    pub const LOST_THRESHOLD: f32 = 25f32;

    /// Create a new [`KeepAliveCounter`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            current: 0,
            send: Timer::from_seconds(Self::PACKET_INTERVAL, TimerMode::Repeating),
            recv: Timer::from_seconds(Self::LOST_THRESHOLD, TimerMode::Once),
        }
    }

    /// Advance the [`KeepAliveCounter`] by `delta` seconds.
    pub fn tick(&mut self, delta: Duration) {
        self.send.tick(delta);
        self.recv.tick(delta);
    }

    /// Returns `true` if a keep-alive packet should be sent.
    #[must_use]
    pub fn should_send(&self) -> bool { self.send.just_finished() }

    /// Returns `true` if the connection is considered lost.
    #[must_use]
    pub fn is_lost(&self) -> bool { self.recv.just_finished() }

    /// Increment the keep-alive counter.
    #[must_use]
    pub fn next_keepalive(&mut self) -> u64 {
        let value = self.current;
        self.current = self.current.wrapping_add(1);
        self.send.set_elapsed(Duration::ZERO);

        value
    }

    /// Receives a keep-alive value.
    ///
    /// Returns `true` if the value is valid.
    #[must_use]
    pub fn receive_keepalive(&mut self, value: u64) -> bool {
        if self.current.wrapping_sub(value) <= Self::VALUE_THRESHOLD {
            self.recv.set_elapsed(Duration::ZERO);
            true
        } else {
            false
        }
    }

    /// A system that adds [`KeepAliveCounter`] components to
    /// new client connections.
    pub fn add_keepalives<V: Version>(
        query: Query<(), With<KeepAliveCounter>>,
        mut events: EventReader<LoginStateEvent<V>>,
        mut commands: Commands,
    ) where
        Clientbound: NetworkDirection<V, Login>,
        Login: State<V>,
    {
        for event in events.read() {
            if !query.contains(event.entity) {
                commands.entity(event.entity).insert(KeepAliveCounter::new());
            }
        }
    }

    /// A system that sends keep-alive packets.
    #[expect(clippy::type_complexity)]
    pub fn tick_keepalives<V: Version + KeepAliveTrait>(
        mut query: Query<(
            Entity,
            &GameProfile,
            &mut KeepAliveCounter,
            Option<&ConfigTask<V>>,
            Option<&PlayTask<V>>,
        )>,
        time: Res<Time<Real>>,
        mut commands: Commands,
    ) where
        Clientbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
        Configuration: State<V>,
        Play: State<V>,
    {
        for (entity, profile, mut keepalive, config, play) in &mut query {
            keepalive.tick(time.delta());

            // If the connection is lost, despawn the entity.
            if keepalive.is_lost() {
                warn!("Connection timed out for {}", profile.username);
                debug!("Despawning Entity {entity}");
                commands.entity(entity).despawn_recursive();
                continue;
            }

            // Otherwise, send a keep-alive packet if necessary.
            if keepalive.should_send() {
                if let Some(config) = config {
                    V::send_config(&mut keepalive, config);
                } else if let Some(play) = play {
                    V::send_play(&mut keepalive, play);
                }
            }
        }
    }

    /// A system that receives keep-alive packets.
    pub fn receive_keepalives<V: Version + KeepAliveTrait>(
        mut query: Query<(&GameProfile, &mut KeepAliveCounter)>,
        mut config: EventReader<ConfigPacketEvent<V>>,
        mut play: EventReader<PlayClientPacketEvent<V>>,
        mut commands: Commands,
    ) where
        Clientbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
        Configuration: State<V>,
        Play: State<V>,
    {
        // Receive keep-alive packets from configuration tasks.
        for event in config.read() {
            if let Ok((profile, mut keepalive)) = query.get_mut(event.entity) {
                if let Some(result) = V::recv_config(&mut keepalive, event) {
                    if !result {
                        warn!("Invalid keep-alive received from {}", profile.username);
                        debug!("Despawning Entity {}", event.entity);
                        commands.entity(event.entity).despawn_recursive();
                    }
                }
            }
        }

        // Receive keep-alive packets from play tasks.
        for event in play.read() {
            if let Ok((profile, mut keepalive)) = query.get_mut(event.entity) {
                if let Some(result) = V::recv_play(&mut keepalive, event) {
                    if !result {
                        warn!("Invalid keep-alive received from {}", profile.username);
                        debug!("Despawning Entity {}", event.entity);
                        commands.entity(event.entity).despawn_recursive();
                    }
                }
            }
        }
    }
}
