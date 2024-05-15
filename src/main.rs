use bevy::prelude::*;

use crate::ambient_light::AmbientLightPlugin;
use crate::app_user_input::AppUserInput;
use crate::asset_loader::AssetLoaderPlugin;
use crate::camera::CameraPlugin;
use crate::collision_detection::CollisionPlugin;
use crate::debug::DebugPlugin;
use crate::despawner::despawn_when_far;
use crate::movement::MovementPlugin;
use crate::spaceship::SpaceshipPlugin;

mod spaceship;
mod movement;
mod debug;
mod camera;
mod ambient_light;
mod asteroids;
mod asset_loader;
mod app_user_input;
mod collision_detection;
mod despawner;

fn main() {
    App::new()
        .add_plugins(AppUserInput)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(AmbientLightPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(SpaceshipPlugin)
        // .add_plugins(AsteroidPlugin)
        .add_systems(Update, despawn_when_far)
        // .add_systems(Update, print_missiles)
        .run();
}

// fn print_missiles(query: Query<Entity, With<SpaceshipMissile>>){
//     println!("Missiles: ");
//     for missile in &query {
//         println!("{}",missile.index());
//     }
// }