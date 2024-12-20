use bevy::{
    prelude::*,
    reflect::{PartialReflect, TypeInfo},
};

/// A set of [`Component`]s received from a [`SubApp`].
#[derive(Debug, Component)]
pub struct SubAppComponents {
    components: Vec<Box<dyn PartialReflect>>,
}

impl Default for SubAppComponents {
    fn default() -> Self { Self::new_empty() }
}

impl SubAppComponents {
    /// Create an empty [`SubAppComponents`].
    #[must_use]
    pub const fn new_empty() -> Self { Self { components: Vec::new() } }

    /// Create a [`SubAppComponents`] from a single [`Component`].
    #[must_use]
    pub fn from_component(component: impl Component + PartialReflect + 'static) -> Self {
        let mut components = Self::new_empty();
        components.push(component);
        components
    }

    /// Add a [`Component`] to the set.
    pub fn push<C: Component + PartialReflect + 'static>(&mut self, component: C) {
        self.push_dyn(Box::new(component));
    }

    /// Add a [`Component`] to the set.
    ///
    /// # Warning
    /// This will cause errors if the reflected type is not a [`Component`].
    pub fn push_dyn(&mut self, reflect: Box<dyn PartialReflect>) { self.components.push(reflect); }
}

impl SubAppComponents {
    /// Collect all [`Component`]s from an [`Entity`].
    #[must_use]
    pub fn read_from(entity: Entity, world: &World) -> Self {
        let registry = world.resource::<AppTypeRegistry>().read();

        let mut components = Self::default();
        let world_entity = world.entity(entity);

        for id in world_entity.archetype().components() {
            // Get the `ComponentInfo` for the component.
            let Some(comp_info) = world.components().get_info(id) else {
                error!("Failed to get ComponentInfo for component: {id:?}");
                continue;
            };

            // Get the `TypeRegistration` for the component.
            let Some(type_reg) = comp_info.type_id().and_then(|t| registry.get(t)) else {
                warn!("Failed to get TypeRegistration for {}", comp_info.name());
                continue;
            };

            // Get the component as a `dyn Reflect`
            let Some(old_reflect) =
                type_reg.data::<ReflectComponent>().and_then(|r| r.reflect(world_entity))
            else {
                warn!(
                    "Failed to get ReflectComponent for type: {}",
                    type_reg.type_info().type_path_table().short_path()
                );
                continue;
            };

            #[cfg(debug_assertions)]
            trace!(
                "Reading SubApp Component: \"{}\"",
                type_reg.type_info().type_path_table().short_path()
            );

            // Clone using `ReflectFromReflect` if available, otherwise use `clone_value`.
            let new_reflect = type_reg
                .data::<ReflectFromReflect>()
                .and_then(|r| r.from_reflect(old_reflect.as_partial_reflect()))
                .map_or_else(|| old_reflect.clone_value(), PartialReflect::into_partial_reflect);

            components.components.push(new_reflect);
        }

        components
    }

    /// Insert all [`Component`]s into an [`Entity`].
    pub fn write_to(self, entity: &mut EntityWorldMut) {
        let registry = entity
            .world_scope::<AppTypeRegistry>(|world| world.resource::<AppTypeRegistry>().clone());
        let registry = registry.read();

        for component in self.components {
            let Some(type_reg) = component
                .get_represented_type_info()
                .map(TypeInfo::type_id)
                .and_then(|id| registry.get(id))
            else {
                warn!(
                    "Failed to get TypeRegistration for component: {}",
                    component.reflect_short_type_path()
                );
                continue;
            };

            if let Some(reflect) = type_reg.data::<ReflectComponent>() {
                #[cfg(debug_assertions)]
                trace!(
                    "Writing SubApp Component: \"{}\"",
                    type_reg.type_info().type_path_table().short_path()
                );

                reflect.apply_or_insert(entity, component.as_partial_reflect(), &registry);
            } else {
                warn!(
                    "Failed to get ReflectComponent for type: {}",
                    type_reg.type_info().type_path_table().short_path()
                );
            }
        }
    }
}
