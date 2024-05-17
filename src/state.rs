use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.add_systems(Update, game_state_input_events);
    }
}

#[derive(States, Copy, Clone, Default, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    GameOver,
}

fn game_state_input_events(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    button_input: Res<ButtonInput<KeyCode>>,
) {
    if button_input.just_pressed(KeyCode::KeyP) {
        println!("Current GameState: {:?}", state.get());
        match state.get() {
            GameState::InGame => {
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                next_state.set(GameState::InGame);
            }
            _ => ()
        }
    }
}