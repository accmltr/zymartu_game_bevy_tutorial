use bevy::prelude::*;
use crate::health::Health;
use crate::schedule::InGameSystemSet;
use crate::state::GameState;

const DESPAWN_DISTANCE: f32 = 100.0;

#[derive(Component, Debug)]
pub struct Despawnable;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::GameOver),
                despawn_all_entities,
            )
            .add_systems(
                Update,
                despawn_when_far.in_set(InGameSystemSet::DespawnEntities),
            );
    }
}

fn despawn_when_far(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), With<Despawnable>>,
) {
    for (entity, global_transform) in &query {
        let distance = global_transform.translation().length();
        if distance >= DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_all_entities(
    mut commands: Commands,
    query: Query<Entity, With<Health>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}