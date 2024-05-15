use bevy::prelude::*;

const DESPAWN_DISTANCE: f32 = 100.0;

pub fn despawn_when_far(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform)>,
) {
    for (entity, global_transform) in &query {
        if global_transform.translation().length() >= DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}