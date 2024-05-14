use std::f32::consts::PI;
use bevy::app::App;
use bevy::pbr::{CascadeShadowConfig, CascadeShadowConfigBuilder};
use bevy::prelude::*;

pub struct AmbientLightPlugin;

impl Plugin for AmbientLightPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            ClearColor(
                Color::rgba(0.12, 0.0, 0.10, 1.0)
            )
        );
        app.insert_resource(
            AmbientLight {
                color: Color::WHITE,
                brightness: 200.0,
            }
        );
        app.add_systems(Startup, spawn_directional_light);
    }
}

fn spawn_directional_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        directional_light: DirectionalLight {
            color: Color::BEIGE,
            illuminance: 7000.0,
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            maximum_distance: 100.0,
            first_cascade_far_bound: 4.0,
            ..default()
        }.into(),
        ..default()
    });
}