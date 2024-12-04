use std::any::{Any, TypeId};

use bevy::{
    app::{AppLabel, InternedAppLabel, Plugins},
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
    /// before all dimensions are registered.
    fn add_dimension(&mut self, dimension: impl DimensionType);

    /// Runs a function on either all or one specific dimension.
    ///
    /// This is useful when you want to run a function on a dimension,
    /// but it does not meet the requirements of any of the other trait methods.
    fn in_dimension(&mut self, dimension: impl DimensionType, f: impl Fn(&mut SubApp));

    /// Runs a function on all dimensions.
    ///
    /// This is useful when you want to run a function on all dimensions,
    /// and want to know which dimension you are working with.
    fn in_all_dimensions(&mut self, f: impl Fn(InternedAppLabel, &mut SubApp));

    /// Initializes a resource for a specific dimension.
    ///
    /// If the dimension is [`All`],
    /// the resource will be initialized for all dimensions.
    fn init_dimension_resource<R: Resource + FromWorld>(&mut self, dimension: impl DimensionType) {
        self.in_dimension(dimension, |sub_app| {
            sub_app.init_resource::<R>();
        });
    }

    /// Inserts a resource into a specific dimension.
    ///
    /// If the dimension is [`All`],
    /// the resource will be inserted into all dimensions.
    fn insert_dimension_resource<R: Resource + Clone>(
        &mut self,
        dimension: impl DimensionType,
        resource: R,
    ) {
        self.in_dimension(dimension, |sub_app| {
            sub_app.insert_resource(resource.clone());
        });
    }

    /// Adds systems to specific dimensions.
    ///
    /// If the dimension is [`All`],
    /// the systems will be added to all dimensions.
    fn add_dimension_systems<M>(
        &mut self,
        dimension: impl DimensionType,
        schedule: impl ScheduleLabel + Clone,
        systems: impl IntoSystemConfigs<M> + Copy,
    ) {
        self.in_dimension(dimension, |sub_app| {
            sub_app.add_systems(schedule.clone(), systems);
        });
    }

    /// Adds an event to specific dimensions.
    ///
    /// If the dimension is [`All`],
    /// the event will be added to all dimensions.
    fn add_dimension_event<E: Event>(&mut self, dimension: impl DimensionType) {
        self.in_dimension(dimension, |sub_app| {
            sub_app.add_event::<E>();
        });
    }

    /// Adds plugins to specific dimensions.
    ///
    /// If the dimension is [`All`],
    /// the plugins will be added to all dimensions.
    fn add_dimension_plugins<M>(
        &mut self,
        dimension: impl DimensionType,
        plugins: impl Plugins<M> + Clone,
    ) {
        self.in_dimension(dimension, |sub_app| {
            sub_app.add_plugins(plugins.clone());
        });
    }
}

impl DimensionApp for App {
    fn add_dimension(&mut self, dimension: impl DimensionType) {
        debug!("Building SubApp for dimension \"{dimension:?}\"");
        let mut sub_app = super::build_subapp(self);
        sub_app.insert_resource(DimensionIdentifier::from(dimension.intern()));

        self.insert_sub_app(dimension, sub_app);
    }

    fn in_dimension(&mut self, dimension: impl DimensionType, f: impl Fn(&mut SubApp)) {
        if dimension.type_id() == TypeId::of::<All>() {
            self.in_all_dimensions(|_, sub_app| f(sub_app));
        } else {
            f(self.sub_app_mut(dimension));
        }
    }

    fn in_all_dimensions(&mut self, f: impl Fn(InternedAppLabel, &mut SubApp)) {
        let dimensions: Vec<_> = self
            .world()
            .resource::<AppTypeRegistry>()
            .read()
            .iter_with_data::<ReflectDimension>()
            .filter_map(|(_, d)| d.app_label)
            .collect();

        for dimension in dimensions {
            f(dimension, self.sub_app_mut(dimension));
        }
    }
}
