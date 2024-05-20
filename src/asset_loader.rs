use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub missiles: Handle<Scene>,
}

#[derive(Resource, Debug, Default)]
pub struct AudioAssets {
    pub asteroid_explosion: Handle<AudioSource>,
    pub fire_missile: Handle<AudioSource>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SceneAssets>()
            .init_resource::<AudioAssets>()
            .add_systems(Startup, load_scene_assets)
            .add_systems(Startup, load_audio_assets)
            .add_systems(PostStartup, play_sound);
    }
}

fn load_scene_assets(
    mut scene_assets: ResMut<SceneAssets>,
    asset_server: Res<AssetServer>,
) {
    *scene_assets = SceneAssets {
        asteroid: asset_server.load("Ultimate Space Kit-glb/Planet.glb#Scene0"),
        spaceship: asset_server.load("Ultimate Space Kit-glb/Spaceship.glb#Scene0"),
        missiles: asset_server.load("Ultimate Space Kit-glb/7.62x39mm.glb#Scene0"),
    }
}

fn load_audio_assets(
    mut audio_assets: ResMut<AudioAssets>,
    asset_server: Res<AssetServer>,
) {
    *audio_assets = AudioAssets {
        asteroid_explosion: asset_server.load("audio/asteroid_explosion.ogg"),
        fire_missile: asset_server.load("audio/fire_missile.ogg"),
    }
}

fn play_sound(
    audio_assets: Res<AudioAssets>,
    mut commands: Commands,
) {
    commands.spawn(AudioBundle {
        source: audio_assets.asteroid_explosion.clone(),
        ..default()
    });
}