//! TODO

use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{network::connection::NetworkDirection, prelude::*};

// use crate::network::play::{PlayClientPacketEvent, PlayTask};

mod ground;
pub use ground::ClientGrounded;

mod version;
pub use version::MovementTrait;

mod systemset;
pub use systemset::MovementSystemSet;

mod teleport;
pub use teleport::TeleportCounter;

use crate::{
    dimension::{subapp::MainAppMarker, All, DimensionApp, Network},
    network::play::PlayClientPacketEvent,
};

/// A [`Plugin`] that manages player movement.
#[derive(Debug, Default)]
pub struct PlayerMovementPlugin<V: Version>(PhantomData<V>);

impl<V: Version + MovementTrait> Plugin for PlayerMovementPlugin<V>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    fn build(&self, app: &mut App) {
        app.register_type::<ClientGrounded>();
        app.register_type::<TeleportCounter>();

        app.in_dimension(All, Self::sub_build);
    }
}

impl<V: Version + MovementTrait> PlayerMovementPlugin<V>
where
    Clientbound: NetworkDirection<V, Play>,
    Play: State<V>,
{
    fn sub_build(app: &mut SubApp) {
        // Only configure `MovementSystemSet` if it doesn't already exist.
        if !app
            .world()
            .resource::<Schedules>()
            .get(Network)
            .is_some_and(|s| s.graph().contains_set(MovementSystemSet))
        {
            app.configure_sets(Network, MovementSystemSet.ambiguous_with_all());
        }

        app.add_systems(
            Network,
            (
                Self::initialize_player_movement.before(Self::receive_player_movement),
                Self::receive_player_movement.run_if(on_event::<PlayClientPacketEvent<V>>),
                Self::update_player_chunk_center.after(Self::receive_player_movement),
            )
                .in_set(MovementSystemSet),
        );
    }

    #[expect(clippy::complexity)]
    fn initialize_player_movement(
        query: Query<
            (Entity, &Transform, Option<&ClientGrounded>),
            (With<MainAppMarker>, Without<TeleportCounter>),
        >,
        mut commands: Commands,
    ) {
        for (entity, transform, grounded) in &query {
            let mut counter = TeleportCounter::default();
            let teleport_id = counter.create_new(*transform);

            V::send_teleport(entity, teleport_id, transform, &mut commands);
            let mut commands = commands.entity(entity);

            commands.insert(counter);
            if grounded.is_none() {
                commands.insert(ClientGrounded::default());
            }
        }
    }

    fn receive_player_movement(
        mut query: Query<(&mut Transform, &mut ClientGrounded, &mut TeleportCounter)>,
        mut events: EventReader<PlayClientPacketEvent<V>>,
    ) {
        for event in events.read() {
            let Ok((mut transform, mut grounded, mut counter)) = query.get_mut(event.entity) else {
                continue;
            };

            if let Some((new_t, new_g)) = V::receive_movement(&transform, &grounded, event) {
                // Handle movement packets
                if new_t != *transform {
                    *transform = new_t;
                }
                if new_g != *grounded {
                    *grounded = new_g;
                }
            } else if let Some(teleport_id) = V::recv_teleport(event) {
                // Handle teleport packets
                if let Some(new_t) = counter.accept(teleport_id) {
                    *transform = new_t;
                }
            }
        }
    }

    #[expect(clippy::cast_possible_truncation)]
    fn update_player_chunk_center(
        mut query: Query<(Entity, &Transform, &mut ChunkPosition), Changed<Transform>>,
        mut commands: Commands,
    ) {
        for (entity, transform, mut center) in &mut query {
            let new_c = ChunkPosition::new(
                transform.translation.x as i64 / i64::from(Chunk::WIDTH),
                transform.translation.z as i64 / i64::from(Chunk::DEPTH),
            );

            if *center != new_c {
                *center = new_c;
                V::send_center(entity, new_c, &mut commands);
            }
        }
    }
}
