use bevy::prelude::*;
use crate::schedule::InGameSystemSet;

#[derive(Component, Debug)]
pub struct Health {
    pub value: f32,
}

impl Health {
    pub fn new(value: f32) -> Self {
        Self {
            value
        }
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                check_health.in_set(InGameSystemSet::DespawnEntities),
            );
    }
}

fn check_health(
    mut commands: Commands,
    query: Query<(Entity, &Health)>,
) {
    for (entity, health) in query.iter() {
        if health.value <= 0f32 {
            commands.entity(entity).despawn_recursive();
        }
    }
}