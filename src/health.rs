use bevy::prelude::*;
use crate::asset_loader::AudioAssets;
use crate::asteroids::Asteroid;
use crate::schedule::InGameSystemSet;

#[derive(Component, Debug)]
pub struct Health {
    pub value: f32,
}

impl Health {
    pub fn new(value: f32) -> Self {
        Self {
            value
        }
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                check_health.in_set(InGameSystemSet::DespawnEntities),
            );
    }
}

fn check_health(
    mut commands: Commands,
    query: Query<(Entity, &Health), With<Asteroid>>,
    audio_assets: Res<AudioAssets>,
) {
    for (entity, health) in query.iter() {
        if health.value <= 0f32 {
            Asteroid::handle_death(&mut commands, &audio_assets);
            commands.entity(entity).despawn_recursive();
        }
    }
}