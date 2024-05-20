use std::f32::consts::PI;
use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::asset_loader::{AudioAssets, SceneAssets};
use crate::collision_damage::CollisionDamage;
use crate::collision_detection::Collider;
use crate::despawner::Despawnable;
use crate::health::Health;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use crate::schedule::InGameSystemSet;

const VELOCITY_SCALAR: f32 = 5.0;
const ACCELERATION_SCALAR: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;
const ASTEROID_RADIUS: f32 = 3.0;
const ASTEROID_RADIUS_RANDOM: f32 = 3.0;
const ASTEROID_ROTATION_SPEED: f32 = 0.2 * PI;
const HEALTH: f32 = 80.0;
const COLLISION_DAMAGE: f32 = 35.0;


#[derive(Component, Debug)]
pub struct Asteroid;

impl Asteroid {
    pub fn handle_death(
        commands: &mut Commands,
        audio_assets: &Res<AudioAssets>,
    ) {
        commands.spawn(AudioBundle {
            source: audio_assets.asteroid_explosion.clone(),
            ..default()
        });
    }
}

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        });
        // app.add_systems(
        //     Update,
        //     handle_asteroid_collisions
        //         .in_set(InGameSystemSet::CollisionDetection),
        // );

        app.add_systems(
            Update,
            (
                asteroid_spawner,
                rotate_asteroid,
            ).chain()
                .in_set(InGameSystemSet::EntityUpdates),
        );
    }
}

fn asteroid_spawner(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
) {
    spawn_timer.timer.tick(time.delta());

    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.0,
        rng.gen_range(SPAWN_RANGE_Z),
    );

    let mut random_unit_vector = || Vec3::new(
        rng.gen_range(-1.0..1.0),
        0.0,
        rng.gen_range(-1.0..1.0),
    ).normalize_or_zero();

    let velocity = VELOCITY_SCALAR * random_unit_vector();
    let acceleration = ACCELERATION_SCALAR * random_unit_vector();

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(velocity),
            acceleration: Acceleration::new(acceleration),
            model: SceneBundle {
                scene: scene_assets.asteroid.clone(),
                transform: Transform::from_translation(translation),
                ..default()
            },
            collider: Collider::new(
                ASTEROID_RADIUS + rng.gen_range(
                    -ASTEROID_RADIUS_RANDOM..ASTEROID_RADIUS_RANDOM
                )
            ),
        },
        Health::new(HEALTH),
        CollisionDamage::new(COLLISION_DAMAGE),
        Asteroid,
        Despawnable,
    ));
}

fn rotate_asteroid(
    mut query: Query<&mut Transform, With<Asteroid>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        transform.rotate_local_z(ASTEROID_ROTATION_SPEED * time.delta_seconds());
    }
}

// fn handle_asteroid_collisions(
//     mut commands: Commands,
//     query_asteroids: Query<(Entity, &Collider), With<Asteroid>>,
//     query_missiles: Query<Entity, With<SpaceshipMissile>>,
// ) {
//     for (entity, collider) in &query_asteroids {
//         for missile_entity in &query_missiles {
//             if collider.new_collisions.contains(&missile_entity) {
//                 commands.entity(entity).despawn_recursive();
//             }
//         }
//     }
// }