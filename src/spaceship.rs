use std::f32::consts::PI;
use std::time::Duration;
use bevy::prelude::*;
use crate::asset_loader::SceneAssets;
use crate::collision_detection::Collider;
use crate::movement::*;

const STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_MISSILE_OFFSET: f32 = 8.0;
const SPACESHIP_MISSILE_SPEED: f32 = 30.0;
const SPACESHIP_RELOAD_TIME: f32 = 0.1;

#[derive(Component, Debug)]
pub struct Spaceship {
    pub last_fire_time: Duration,
}

impl Spaceship {
    pub fn new() -> Self {
        Spaceship { last_fire_time: Duration::ZERO }
    }
}

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship);
        app.add_systems(Update, spaceship_movement_controls);
        app.add_systems(Update, spaceship_weapon_controls);
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SceneBundle {
                scene: scene_assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_POSITION),
                ..default()
            },
            collider: Collider::new(7.0),
        },
        Spaceship::new(),
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    button_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.0f32;
    let mut roll = 0.0f32;
    let mut movement = 0.0f32;

    if button_input.pressed(KeyCode::KeyW) {
        movement = SPACESHIP_SPEED;
    }
    if button_input.pressed(KeyCode::KeyS) {
        movement = -SPACESHIP_SPEED;
    }
    velocity.value = -transform.forward() * movement;

    if button_input.pressed(KeyCode::KeyD) {
        rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }
    if button_input.pressed(KeyCode::KeyA) {
        rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
    }
    transform.rotate_y(rotation);

    if button_input.pressed(KeyCode::KeyE) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }
    if button_input.pressed(KeyCode::KeyQ) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }
    transform.rotate_local_z(roll);
}

fn spaceship_weapon_controls(
    mut commands: Commands,
    mut query: Query<(&Transform, &mut Spaceship), With<Spaceship>>,
    scene_assets: Res<SceneAssets>,
    time: Res<Time>,
    button_input: Res<ButtonInput<KeyCode>>,
) {
    let (transform, mut spaceship) = query.single_mut();

    if time.elapsed() - spaceship.last_fire_time < Duration::from_secs_f32(SPACESHIP_RELOAD_TIME) {
        return;
    }

    if button_input.pressed(KeyCode::Space) {
        let mut bullet_transform = Transform::from_translation(
            transform.translation - transform.forward() * SPACESHIP_MISSILE_OFFSET
        ).with_scale(Vec3::ONE * 7.0)
            .looking_to(transform.forward().xyz(), Vec3::Z);
        bullet_transform.rotate_local_x(PI / 2.0);
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-transform.forward() * SPACESHIP_MISSILE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                model: SceneBundle {
                    scene: scene_assets.missiles.clone(),
                    transform: bullet_transform,
                    ..default()
                },
                collider: Collider::new(1.0),
            },
            SpaceshipMissile
        ));
        spaceship.last_fire_time = time.elapsed();
    }
}