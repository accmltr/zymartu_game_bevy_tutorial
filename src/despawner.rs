use bevy::prelude::*;

const DESPAWN_DISTANCE: f32 = 100.0;

#[derive(Component, Debug)]
pub struct Despawnable;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_when_far);
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