use bevy::app::App;
use bevy::prelude::*;

pub struct AmbientLightPlugin;

impl Plugin for AmbientLightPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            ClearColor(
                Color::rgba(0.1, 0.0, 0.15, 1.0)
            )
        );
        app.insert_resource(
            AmbientLight {
                color: Color::WHITE,
                brightness: 1000.0,
            }
        );
    }
}