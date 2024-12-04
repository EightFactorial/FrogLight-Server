use bevy::prelude::*;
use froglight::{network::connection::NetworkDirection, prelude::*};

use super::{
    CompletedPlay, PlayClientPacketEvent, PlayPacketEventQueue, PlayRequiredComponents,
    PlayServerPacketEvent, PlayTask, PlayTrait,
};
use crate::{
    dimension::subapp::{DimensionIdentifier, DimensionMarker, DimensionTracker, MainAppMarker},
    network::{common::channel, config::ConfigStateEvent},
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
    #[expect(clippy::missing_panics_doc)]
    pub fn receive_configured(
        query: Query<&GameProfile>,
        mut events: EventReader<ConfigStateEvent<V>>,
        mut commands: Commands,
    ) {
        for ConfigStateEvent { entity, connection } in events.read() {
            if let Some(conn) = connection.lock().take() {
                debug!("Starting play session for {} ...", query.get(*entity).unwrap().name);
                commands.entity(*entity).insert(PlayTask::<V>::new(conn));
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
                debug!("Sending reconfigure to {}", profile.name);
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
        query: Query<(&DimensionMarker, &DimensionTracker, &PlayTask<V>)>,
        queue: ResMut<PlayPacketEventQueue<V>>,
    ) {
        {
            // Receive clientbound packets
            let mut queue = queue.client.lock();
            for (marker, tracker, task) in &query {
                while let Some(packet) = task.recv() {
                    queue
                        .entry(**marker)
                        .or_default()
                        .push(PlayClientPacketEvent::new(**tracker, packet));
                }
            }
        }
        {
            // Send serverbound packets
            let mut queue = queue.server.lock();
            for PlayServerPacketEvent { entity, packet } in queue.drain(..) {
                if let Ok((_, _, task)) = query.get(entity) {
                    task.send_arc(packet);
                } else {
                    warn!("Received packet for non-existent connection!");
                }
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
        mut server: EventReader<PlayServerPacketEvent<V>>,
    ) {
        // Receive serverbound packets
        client.send_batch(queue.client.lock().entry(**label).or_default().drain(..));

        // Send clientbound packets
        let mut queue = queue.server.lock();
        for PlayServerPacketEvent { entity, packet } in server.read() {
            if let Ok(marker) = query.get(*entity) {
                queue.push(PlayServerPacketEvent { entity: **marker, packet: packet.clone() });
            } else {
                warn!("Received packet for non-existent connection!");
            }
        }
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
                    debug!("Reconfiguring {}", profile.name);
                    commands.entity(entity).remove::<PlayTask<V>>();
                }
                Some(Err(ConnectionError::ConnectionClosed)) => {
                    info!("Disconnected {}", profile.name);
                    debug!("Despawning Entity {entity}");
                    commands.entity(entity).despawn_recursive();
                }
                Some(Err(err)) => {
                    error!("Error for {}: {err}", profile.name);
                    debug!("Despawning Entity {entity}");
                    commands.entity(entity).despawn_recursive();
                }
                None => {}
            }
        }
    }
}
