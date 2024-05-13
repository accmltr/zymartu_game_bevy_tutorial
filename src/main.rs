mod spaceship;
mod movement;
mod debug;
mod camera;
mod ambient_light;

use bevy::prelude::*;
use crate::ambient_light::AmbientLightPlugin;
use crate::debug::DebugPlugin;
use crate::movement::MovementPlugin;
use crate::spaceship::SpaceshipPlugin;
use crate::camera::CameraPlugin;


fn main() {
    App::new()
        .add_plugins(AmbientLightPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(SpaceshipPlugin)
        .run();
}

