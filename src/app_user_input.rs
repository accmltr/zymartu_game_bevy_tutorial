use bevy::app::AppExit;
use bevy::prelude::*;

pub struct AppUserInput;

impl Plugin for AppUserInput {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, exit_on_escape);
    }
}

fn exit_on_escape(button_input: Res<ButtonInput<KeyCode>>, mut event_writer: EventWriter<AppExit>) {
    if button_input.just_pressed(KeyCode::Escape) {
        event_writer.send(AppExit);
    }
}