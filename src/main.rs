use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use iyes_perf_ui::{PerfUiCompleteBundle, PerfUiPlugin};

use crate::ambient_light::AmbientLightPlugin;
use crate::app_user_input::AppUserInput;
use crate::asset_loader::AssetLoaderPlugin;
use crate::asteroids::AsteroidPlugin;
use crate::camera::CameraPlugin;
use crate::collision_detection::CollisionPlugin;
use crate::debug::DebugPlugin;
use crate::despawner::DespawnPlugin;
use crate::movement::MovementPlugin;
use crate::schedule::InGameSystemSet;
use crate::spaceship::SpaceshipPlugin;
use crate::state::{GameState, StatePlugin};

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
mod schedule;
mod state;

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
        .add_plugins(DespawnPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_plugins(StatePlugin)
        .add_systems(PostStartup, spawn_performance_ui)
        .configure_sets(
            Update,
            TestSystemSets::Set1
                .run_if(in_state(GameState::Paused))
                .before(InGameSystemSet::UserInput)
        )
        .add_systems(Update, spam.in_set(TestSystemSets::Set1))
        .run();
}

fn spawn_performance_ui(
    mut commands: Commands
) {
    println!("Spawning performance UI");
    commands.spawn(PerfUiCompleteBundle::default());
}

fn spam() {
    println!("Paused");
}

#[derive(SystemSet, Debug, Hash, Eq, PartialEq, Clone)]
enum TestSystemSets {
    Set1,
    Set2
}