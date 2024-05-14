use bevy::input::InputPlugin;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;
use crate::asset_loader::SceneAssets;

use crate::movement::*;

const STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const SPACESHIP_MISSILE_SPEED: f32 = 30.0;

#[derive(Component, Debug)]
pub struct Spaceship;

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
        },
        Spaceship,
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
    query: Query<&Transform, With<Spaceship>>,
    scene_assets: Res<SceneAssets>,
    button_input: Res<ButtonInput<KeyCode>>,
) {
    let transform = query.single();

    if button_input.pressed(KeyCode::Space) {
        commands.spawn(
            MovingObjectBundle {
                velocity: Velocity::new(Vec3::new(0.0, 0.0, SPACESHIP_MISSILE_SPEED)),
                acceleration: Acceleration::new(Vec3::ZERO),
                model: SceneBundle {
                    scene: scene_assets.missiles.clone(),
                    transform: transform.clone().with_scale(3.0f32 * Vec3::ONE),
                    ..default()
                },
            }
        );
    }
}