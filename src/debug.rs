use bevy::prelude::*;
// use bevy::log::info;
// use bevy::prelude::{Entity, Query, Transform};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(Update, print_position);
    }
}


// fn print_position(query: Query<(Entity, &Transform)>) {
//     for (entity, transform) in query.iter() {
//         info!("Entity {:?} is at position {:?}", entity, transform.translation);
//     }
// }