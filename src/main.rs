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

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::prelude::KeyCode::KeyC;
use crate::ambient_light::AmbientLightPlugin;
use crate::app_user_input::AppUserInput;
use crate::asset_loader::AssetLoaderPlugin;
use crate::asteroids::AsteroidPlugin;
use crate::debug::DebugPlugin;
use crate::movement::MovementPlugin;
use crate::spaceship::SpaceshipPlugin;
use crate::camera::CameraPlugin;
use crate::collision_detection::CollisionPlugin;
use crate::despawner::despawn_when_far;


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
        .add_plugins(AsteroidPlugin)
        .add_systems(Update, despawn_when_far)
        .run();
}