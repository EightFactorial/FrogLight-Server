use std::time::Instant;

use bevy::prelude::*;
use froglight::{network::connection::NetworkDirection, prelude::*};

use super::{
    CompletedPlay, PlayClientPacketEvent, PlayPacketEventQueue, PlayRequiredComponents,
    PlayServerPacketEvent, PlayTask, PlayTrait,
};
use crate::{
    dimension::subapp::{DimensionIdentifier, DimensionMarker, MainAppMarker, SubAppTracker},
    network::{common::channel, config::ConfigStateEvent, login::ConnectionInstant},
};

impl<V: Version + PlayTrait> PlayTask<V>
where
    Clientbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    /// Create a new [`PlayTask`] with the given [`Connection`].
    #[must_use]
    pub fn new(conn: Connection<V, Configuration, Clientbound>) -> Self {
        let (send, recv) = channel();
        Self::spawn(send, V::play(conn.play(), recv))
    }

    /// A system that receives configured connections and
    /// starts play sessions for them.
    pub fn receive_configured(
        query: Query<(&GameProfile, Option<&ConnectionInstant>)>,
        mut events: EventReader<ConfigStateEvent<V>>,
        mut commands: Commands,
    ) {
        for ConfigStateEvent { entity, connection } in events.read() {
            if let Ok((profile, instant)) = query.get(*entity) {
                if let Some(conn) = connection.lock().take() {
                    // Start the play session
                    let mut commands = commands.entity(*entity);
                    commands.insert(PlayTask::<V>::new(conn));

                    if let Some(instant) = instant {
                        // If there is an instant, log the session and duration
                        let elapsed = Instant::now().duration_since(**instant);
                        if elapsed.as_secs_f32() >= 0.1 {
                            info!(
                                "Starting play session for {} ({:.2}s)",
                                profile.username,
                                elapsed.as_secs_f32()
                            );
                        } else if elapsed.as_millis() > 0 {
                            info!(
                                "Starting play session for {} ({}ms)",
                                profile.username,
                                elapsed.as_millis()
                            );
                        } else {
                            info!(
                                "Starting play session for {} ({}µs)",
                                profile.username,
                                elapsed.as_micros()
                            );
                        }

                        // Remove the instant
                        commands.remove::<ConnectionInstant>();
                    } else {
                        // Otherwise just log the session
                        info!("Starting play session for {}", profile.username);
                    }
                }
            }
        }
    }

    /// A system that reconfigures all sessions that
    /// have the required components.
    pub fn reconfigure_session(
        query: Query<(Entity, &GameProfile, &PlayTask<V>), Without<CompletedPlay>>,
        required: Res<PlayRequiredComponents<V>>,
        world: &World,
        mut commands: Commands,
    ) {
        for (entity, profile, task) in &query {
            if required.check(entity, world) {
                debug!("Sending reconfigure to {}", profile.username);
                V::send_reconfigure(task);
                commands.entity(entity).insert(CompletedPlay);
            }
        }
    }
}

impl<V: Version> PlayTask<V>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    /// A system that receives serverbound packets from play tasks,
    /// and receives clientbound packets from the queue.
    pub fn app_queue_and_receive_packets(
        query: Query<(Entity, &DimensionMarker, &SubAppTracker, &PlayTask<V>)>,
        queue: ResMut<PlayPacketEventQueue<V>>,
        mut events: EventWriter<PlayClientPacketEvent<V>>,
    ) {
        {
            // Receive clientbound packets
            let mut queue = queue.client.lock();
            for (entity, marker, tracker, task) in &query {
                while let Some(packet) = task.recv() {
                    // Send a copy of the packet event in the main App
                    events.send(PlayClientPacketEvent::new(entity, packet.clone()));
                    // Send the packet event to the SubApp queue
                    queue
                        .entry(***marker)
                        .or_default()
                        .push(PlayClientPacketEvent::new(**tracker, packet));
                }
            }
        }
        {
            // Send serverbound packets
            for PlayServerPacketEvent { entity, packet } in queue.server.lock().drain(..) {
                query.get(entity).map_or_else(
                    |_| warn!("Received packet for non-existent connection!"),
                    |(_, _, _, task)| task.send_arc(packet),
                );
            }
        }
    }

    /// A [`SubApp`] system that receives serverbound packets from the queue,
    /// and sends clientbound packets to the queue.
    pub fn sub_queue_and_receive_packets(
        query: Query<&MainAppMarker>,
        label: Res<DimensionIdentifier>,
        queue: ResMut<PlayPacketEventQueue<V>>,
        mut client: EventWriter<PlayClientPacketEvent<V>>,
        mut server: ResMut<Events<PlayServerPacketEvent<V>>>,
    ) {
        // Receive serverbound packets
        client.send_batch(queue.client.lock().entry(**label).or_default().drain(..));

        // Send clientbound packets
        queue.server.lock().extend(server.drain().filter_map(
            |PlayServerPacketEvent { entity, packet }| {
                query.get(entity).map_or_else(
                    |_| {
                        warn!("Received packet for non-existent connection!");
                        None
                    },
                    |marker| Some(PlayServerPacketEvent { entity: **marker, packet }),
                )
            },
        ));
    }

    /// A system that polls all play tasks and
    /// despawns them if they are done.
    pub fn poll_tasks(
        mut query: Query<(Entity, &GameProfile, &mut PlayTask<V>)>,
        mut commands: Commands,
    ) {
        for (entity, profile, mut task) in &mut query {
            match task.poll() {
                Some(Ok(_conn)) => {
                    debug!("Reconfiguring {}", profile.username);
                    commands.entity(entity).remove::<PlayTask<V>>();
                }
                Some(Err(ConnectionError::ConnectionClosed)) => {
                    info!("Disconnected {}", profile.username);
                    debug!("Despawning Entity {entity}");
                    commands.entity(entity).despawn_recursive();
                }
                Some(Err(err)) => {
                    error!("Error for {}: {err}", profile.username);
                    debug!("Despawning Entity {entity}");
                    commands.entity(entity).despawn_recursive();
                }
                None => {}
            }
        }
    }
}
