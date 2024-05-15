use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub new_collisions: Vec<Entity>,
    pub collisions: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius: radius,
            new_collisions: vec![],
            collisions: vec![],
        }
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut new_data: HashMap<Entity, (Vec<Entity>, Vec<Entity>)> = HashMap::new();
    for (entity_a, transform_a, collider_a) in &query {
        let mut colliding_entities = vec![];
        let mut new_colliding_entities = vec![];
        for (entity_b, transform_b, collider_b) in &query {
            if entity_a != entity_b {
                let distance = transform_a.translation()
                    .distance(transform_b.translation());
                let total_radius = collider_a.radius + collider_b.radius;
                if distance < total_radius {
                    colliding_entities.push(entity_b);

                    if !collider_a.collisions.contains(&entity_b) {
                        new_colliding_entities.push(entity_b);
                    }
                }
            }
        }
        new_data.insert(entity_a, (new_colliding_entities, colliding_entities));
    }

    for (entity, _, mut collider) in &mut query {
        match new_data.get(&entity) {
            Some((new_collisions, collisions)) => {
                collider.new_collisions = new_collisions.clone();
                collider.collisions = collisions.clone();
            }
            _ => {}
        }
    }
}