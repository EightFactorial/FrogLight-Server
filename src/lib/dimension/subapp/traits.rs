use std::any::{Any, TypeId};

use bevy::{
    app::{AppLabel, Plugins},
    ecs::schedule::ScheduleLabel,
    prelude::*,
};

use crate::dimension::{subapp::DimensionIdentifier, ReflectDimension};

/// A trait for types that represent dimensions.
pub trait DimensionType: AppLabel + Default {}

/// A [`DimensionType`] that represents all dimensions.
///
/// When adding plugins or systems to a dimension,
/// using `All` as the dimension will add them to all dimensions.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, AppLabel)]
pub struct All;
impl DimensionType for All {}

/// Adds dimension-related builder methods to [`App`].
pub trait DimensionApp {
    /// Adds a dimension to the app.
    ///
    /// # Note
    /// This only needs to be used if the dimension is not registered *before*
    /// the [`DimensionPlugin`](crate::dimension::DimensionPlugin) is added
    /// to the app.
    ///
    /// Doing this may cause systems and plugins to be missing if they are added
    /// before the dimension is registered.
    fn add_dimension(&mut self, dimension: impl DimensionType);

    /// Adds systems to specific dimensions.
    ///
    /// If the dimension is [`All`],
    /// the systems will be added to all dimensions.
    fn add_dimension_systems<M>(
        &mut self,
        dimension: impl DimensionType,
        schedule: impl ScheduleLabel + Clone,
        systems: impl IntoSystemConfigs<M> + Copy,
    );

    /// Adds plugins to specific dimensions.
    ///
    /// If the dimension is [`All`],
    /// the plugins will be added to all dimensions.
    fn add_dimension_plugins<M>(
        &mut self,
        dimension: impl DimensionType,
        plugins: impl Plugins<M> + Clone,
    );
}

impl DimensionApp for App {
    fn add_dimension(&mut self, dimension: impl DimensionType) {
        debug!("Building SubApp for dimension \"{dimension:?}\"");
        let mut sub_app = super::build_subapp(self);
        sub_app.insert_resource(DimensionIdentifier::from(dimension.intern()));

        self.insert_sub_app(dimension, sub_app);
    }

    fn add_dimension_systems<M>(
        &mut self,
        dimension: impl DimensionType,
        schedule: impl ScheduleLabel + Clone,
        systems: impl IntoSystemConfigs<M> + Copy,
    ) {
        if dimension.type_id() == TypeId::of::<All>() {
            let dimensions: Vec<_> = self
                .world()
                .resource::<AppTypeRegistry>()
                .clone()
                .read()
                .iter_with_data::<ReflectDimension>()
                .filter_map(|(_, d)| d.app_label)
                .collect();

            for dimension in dimensions {
                self.sub_app_mut(dimension).add_systems(schedule.clone(), systems);
            }
        } else {
            self.sub_app_mut(dimension).add_systems(schedule, systems);
        }
    }

    fn add_dimension_plugins<M>(
        &mut self,
        dimension: impl DimensionType,
        plugins: impl Plugins<M> + Clone,
    ) {
        if dimension.type_id() == TypeId::of::<All>() {
            let dimensions: Vec<_> = self
                .world()
                .resource::<AppTypeRegistry>()
                .clone()
                .read()
                .iter_with_data::<ReflectDimension>()
                .filter_map(|(_, d)| d.app_label)
                .collect();

            for dimension in dimensions {
                self.sub_app_mut(dimension).add_plugins(plugins.clone());
            }
        } else {
            self.sub_app_mut(dimension).add_plugins(plugins);
        }
    }
}
