use bevy::prelude::*;
use crate::schedule::InGameSystemSet;
// use bevy::log::info;
// use bevy::prelude::{Entity, Query, Transform};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            print_position
                .after(InGameSystemSet::EntityUpdates),
        );
    }
}


fn print_position(_query: Query<(Entity, &Transform)>) {
    // for (entity, transform) in query.iter() {
    //     info!("Entity {:?} is at position {:?}", entity, transform.translation);
    // }
}