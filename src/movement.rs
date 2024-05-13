use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}


fn update_position(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.value.x * time.delta_seconds();
        transform.translation.y += velocity.value.y * time.delta_seconds();
        transform.translation.z += velocity.value.z * time.delta_seconds();
    }
}
