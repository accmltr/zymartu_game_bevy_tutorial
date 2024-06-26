use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use crate::collision_detection::Collider;
use crate::schedule::InGameSystemSet;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub model: SceneBundle,
    pub collider: Collider,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_velocity,
            update_position
        ).chain()
            .in_set(InGameSystemSet::EntityUpdates));
    }
}


fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.value.x * time.delta_seconds();
        transform.translation.y += velocity.value.y * time.delta_seconds();
        transform.translation.z += velocity.value.z * time.delta_seconds();
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}