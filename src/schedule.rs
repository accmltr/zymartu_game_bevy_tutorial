use bevy::prelude::*;
use crate::schedule::InGameSystemSet::{CollisionDetection, DespawnEntities, EntityUpdates, UserInput};
use crate::state::GameState;

pub struct MySchedulePlugin;

impl Plugin for MySchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                DespawnEntities,
                UserInput,
                EntityUpdates,
                CollisionDetection,
            ).chain().run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(SystemSet, Debug, Hash, Eq, PartialEq, Clone)]
pub enum InGameSystemSet {
    UserInput,
    EntityUpdates,
    CollisionDetection,
    DespawnEntities,
}