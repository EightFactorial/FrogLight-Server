//! TODO

use std::marker::PhantomData;

use bevy::prelude::*;
use froglight::{network::connection::NetworkDirection, prelude::*};

mod systemset;
pub use systemset::SettingsSystemSet;

mod version;
pub use version::SettingsTrait;

use crate::{
    dimension::subapp::{
        DimensionMarker, SubAppComponents, SubAppEvent, SubAppEventQueue, SubAppTracker,
    },
    network::{
        common::FilterResult,
        config::{ConfigFilter, ConfigPacketEvent},
        play::PlayClientPacketEvent,
    },
};

/// A [`Plugin`] that manages player settings.
#[derive(Debug, Default)]
pub struct PlayerSettingsPlugin<V: Version>(PhantomData<V>);

impl<V: Version + SettingsTrait> Plugin for PlayerSettingsPlugin<V>
where
    Clientbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    fn build(&self, app: &mut App) {
        app.register_type::<ClientSettings>();
        app.world_mut().resource_mut::<ConfigFilter<V>>().add_filter(ClientSettings::filter);

        // Only configure `SettingsSystemSet` if it doesn't already exist.
        if !app
            .world()
            .resource::<Schedules>()
            .get(Update)
            .is_some_and(|s| s.graph().contains_set(SettingsSystemSet))
        {
            app.configure_sets(Update, SettingsSystemSet);
        }

        app.add_systems(
            Update,
            ClientSettings::receive_client_settings::<V>
                .run_if(on_event::<PlayClientPacketEvent<V>>.or(on_event::<ConfigPacketEvent<V>>))
                .in_set(SettingsSystemSet),
        );
    }
}

/// A struct that stores the client's settings.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct ClientSettings {
    /// The client's language.
    pub language: String,
    /// The client's view distance.
    pub view_distance: u8,
    /// The client's chat visibility.
    pub chat_visibility: ChatVisibility,
    /// Whether the client's chat colors are enabled.
    pub chat_colors: bool,
    /// The client's model customization flags.
    pub model_customization: PlayerModelFlags,
    /// The client's main hand.
    pub main_hand: PlayerHand,
    /// Whether the client's text filtering is enabled.
    pub text_filtering_enabled: bool,
    /// Whether the client allows appearing on server listing.
    pub allows_listing: bool,
}

/// A marker [`Component`] for connections that have client settings.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[component(storage = "SparseSet")]
#[reflect(Component)]
pub struct HasClientSettings;

impl ClientSettings {
    const DENY_REASON: &'static str = "Client never sent settings";

    /// A filter that requires the connection to have sent settings.
    fn filter(entity: Entity, world: &World) -> FilterResult {
        if world.get::<HasClientSettings>(entity).is_some() {
            FilterResult::Allow
        } else {
            FilterResult::Deny(Some(Self::DENY_REASON.into()))
        }
    }

    /// A system that receives settings packets and
    /// applies the settings component.
    pub fn receive_client_settings<V: Version + SettingsTrait>(
        mut query: Query<(
            Option<&SubAppTracker>,
            Option<&DimensionMarker>,
            Option<&mut SubAppComponents>,
        )>,
        mut config: EventReader<ConfigPacketEvent<V>>,
        mut play: EventReader<PlayClientPacketEvent<V>>,

        mut events: ResMut<SubAppEventQueue>,
        mut commands: Commands,
    ) where
        Clientbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
        Configuration: State<V>,
        Play: State<V>,
    {
        for event in config.read() {
            if let Some(settings) = V::config_settings(event) {
                commands.entity(event.entity).insert(HasClientSettings);
                Self::match_query(settings, event.entity, &mut query, &mut events, &mut commands);
            }
        }

        for event in play.read() {
            if let Some(settings) = V::play_settings(event) {
                commands.entity(event.entity).insert(HasClientSettings);
                Self::match_query(settings, event.entity, &mut query, &mut events, &mut commands);
            }
        }
    }

    fn match_query(
        settings: ClientSettings,
        entity: Entity,
        query: &mut Query<(
            Option<&SubAppTracker>,
            Option<&DimensionMarker>,
            Option<&mut SubAppComponents>,
        )>,
        events: &mut SubAppEventQueue,
        commands: &mut Commands,
    ) {
        match query.get_mut(entity) {
            // Send a `SubAppEvent::InsertComponent` event.
            Ok((Some(tracker), Some(marker), ..)) => {
                if let Some(queue) = events.get_mut(&**marker) {
                    queue.push(SubAppEvent::InsertComponent(*tracker, Box::new(settings)));
                }
            }
            // Add to any existing `SubAppComponents`.
            Ok((.., Some(mut components))) => {
                components.push(settings);
            }
            // Create and insert a new `SubAppComponents`.
            Ok((None, None, None)) => {
                let mut components = SubAppComponents::default();
                components.push(settings);
                commands.entity(entity).insert(components);
            }
            _ => {
                warn!("Failed to apply ClientSettings: Unexpected query results");
            }
        }
    }
}
