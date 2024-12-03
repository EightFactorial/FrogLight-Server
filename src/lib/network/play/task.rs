use bevy::prelude::*;
use froglight::{network::connection::NetworkDirection, prelude::*};

use super::{
    CompletedPlay, PlayPacketEvent, PlayPacketEventQueue, PlayRequiredComponents, PlayTask,
    PlayTrait,
};
use crate::{
    dimension::subapp::{DimensionIdentifier, DimensionMarker, DimensionTracker},
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

    /// A system that completes all sessions that have the required components.
    pub fn complete_play_sessions(
        query: Query<(Entity, &GameProfile, &PlayTask<V>), Without<CompletedPlay>>,
        required: Res<PlayRequiredComponents<V>>,
        world: &World,
        mut commands: Commands,
    ) {
        for (entity, profile, task) in &query {
            if required.check(entity, world) {
                debug!("Ending session for {}", profile.name);
                V::send_finish(task);
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
    /// A system that receives packets from all play tasks
    /// and puts them into the shared packet queue.
    pub fn queue_received_packets(
        query: Query<(&DimensionMarker, &DimensionTracker, &PlayTask<V>)>,
        queue: ResMut<PlayPacketEventQueue<V>>,
    ) {
        let mut queue = queue.write();
        for (marker, tracker, task) in &query {
            while let Some(packet) = task.recv() {
                queue.entry(**marker).or_default().push(PlayPacketEvent::new(**tracker, packet));
            }
        }
    }

    /// A system that receives packet events from the shared packet queue.
    pub fn receive_queued_packets(
        label: Res<DimensionIdentifier>,
        queue: ResMut<PlayPacketEventQueue<V>>,
        mut events: EventWriter<PlayPacketEvent<V>>,
    ) {
        events.send_batch(queue.write().entry(**label).or_default().drain(..));
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
