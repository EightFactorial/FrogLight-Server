//! TODO

use std::any::TypeId;

use bevy::prelude::*;
use froglight::prelude::GameProfile;

use crate::dimension::subapp::{
    DimensionMarker, SubAppComponents, SubAppEvent, SubAppEventQueue, SubAppTracker,
};

/// A plugin for syncing the app and subapp player profiles.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerProfileSyncPlugin;

impl Plugin for PlayerProfileSyncPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameProfile>();

        app.configure_sets(Update, ProfileSyncSystemSet);
        app.add_systems(
            Update,
            (
                PlayerProfileSyncPlugin::add_profile_component,
                PlayerProfileSyncPlugin::sync_profile_to_subapp,
            )
                .run_if(any_with_component::<GameProfile>),
        );
    }
}

/// A [`SystemSet`] that for systems that sync player profiles.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct ProfileSyncSystemSet;

impl PlayerProfileSyncPlugin {
    /// A system that adds a player profile component to the list of
    /// subapp components.
    pub fn add_profile_component(
        mut query: Query<(Entity, &GameProfile, Option<&mut SubAppComponents>), Added<GameProfile>>,
        mut commands: Commands,
    ) {
        for (entity, profile, components) in &mut query {
            if let Some(mut components) = components {
                components.push(profile.clone());
            } else {
                commands.entity(entity).insert(SubAppComponents::from_component(profile.clone()));
            }
        }
    }

    /// A system that syncs any changes to the player profile
    /// in the main app to the linked entity in a subapp.
    pub fn sync_profile_to_subapp(
        query: Query<(&DimensionMarker, &SubAppTracker, &GameProfile), Changed<GameProfile>>,
        registry: Res<AppTypeRegistry>,
        mut queue: ResMut<SubAppEventQueue>,
    ) {
        if query.is_empty() {
            return;
        }

        let registry = registry.read();
        let Some(profile_reg) =
            registry.get_type_data::<ReflectFromReflect>(TypeId::of::<GameProfile>())
        else {
            return;
        };

        for (marker, tracker, profile) in &query {
            if let Some(profile) = profile_reg.from_reflect(profile.as_partial_reflect()) {
                queue
                    .entry(**marker)
                    .or_default()
                    .push(SubAppEvent::InsertComponent(*tracker, profile.into_partial_reflect()));
            }
        }
    }
}
