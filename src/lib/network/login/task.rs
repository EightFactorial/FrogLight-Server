use bevy::{prelude::*, utils::HashMap};
use froglight::{
    network::connection::{AccountInformation, NetworkDirection},
    prelude::{State, *},
};

use super::{
    AuthenticationServer, LoginPacketEvent, LoginRequiredComponents, LoginTask, LoginTrait,
};
use crate::network::{
    common::channel,
    login::{CompletedLogin, LoginStateEvent},
    socket::ConnectionRequestEvent,
};

impl<V: Version + LoginTrait> LoginTask<V>
where
    Clientbound: NetworkDirection<V, Login>,
    Login: State<V>,
{
    /// Create a new [`LoginTask`] with the
    /// given [`Connection`] and [`AuthenticationServer`].
    #[must_use]
    pub fn new(
        conn: Connection<V, Login, Clientbound>,
        auth_server: AuthenticationServer<V>,
        resolver: Resolver,
    ) -> Self {
        let (send, recv) = channel();
        Self::spawn(send, V::login(conn, recv, auth_server, resolver))
    }

    /// A system that authenticates incoming connection requests.
    pub fn receive_requests(
        mut events: EventReader<ConnectionRequestEvent<V>>,
        auth: Res<AuthenticationServer<V>>,
        resolver: Res<Resolver>,
        mut commands: Commands,
    ) {
        for ConnectionRequestEvent { listener, request } in events.read() {
            if let Some(conn) = request.connection.lock().take() {
                debug!("Logging in {} ...", request.username);
                let mut entity = commands.spawn((
                    request.information.clone(),
                    GameProfile {
                        uuid: if auth.read().is_none() {
                            AccountInformation::offline_uuid(&request.username)
                        } else {
                            request.uuid
                        },
                        name: request.username.clone(),
                        properties: HashMap::new(),
                    },
                    LoginTask::<V>::new(conn, auth.clone(), resolver.clone()),
                ));

                entity.set_parent(*listener);
                debug!("Spawning Entity {} for {}", entity.id(), request.username);
            }
        }
    }

    /// A system that receives packets from all login tasks.
    pub fn receive_packets(
        query: Query<(Entity, &LoginTask<V>)>,
        mut events: EventWriter<LoginPacketEvent<V>>,
    ) {
        for (entity, task) in &query {
            while let Some(packet) = task.recv() {
                events.send(LoginPacketEvent::new(entity, packet));
            }
        }
    }

    /// A system that completes all logins that have the required components.
    pub fn complete_logins(
        query: Query<(Entity, &GameProfile, &LoginTask<V>), Without<CompletedLogin>>,
        required: Res<LoginRequiredComponents<V>>,
        world: &World,
        mut commands: Commands,
    ) {
        for (entity, profile, task) in &query {
            if required.check(entity, world) {
                debug!("Sending profile to {}", profile.name);
                V::send_profile(profile, task);
                commands.entity(entity).insert(CompletedLogin);
            }
        }
    }

    /// A system that polls all login tasks and
    /// despawns them if they are done.
    pub fn poll_tasks(
        mut query: Query<(Entity, &GameProfile, &mut LoginTask<V>)>,
        mut events: EventWriter<LoginStateEvent<V>>,
        mut commands: Commands,
    ) {
        for (entity, profile, mut task) in &mut query {
            match task.poll() {
                Some(Ok(conn)) => {
                    info!("Logged in {}", profile.name);
                    commands.entity(entity).remove::<LoginTask<V>>();
                    events.send(LoginStateEvent::<V>::new(entity, conn));
                }
                Some(Err(err)) => {
                    error!("Login failed for {}: {err}", profile.name);
                    debug!("Despawning Entity {entity}");
                    commands.entity(entity).despawn_recursive();
                }
                None => {}
            }
        }
    }
}
