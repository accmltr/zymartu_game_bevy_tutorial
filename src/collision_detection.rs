use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::asteroids::Asteroid;
use crate::collision_damage::CollisionDamage;
use crate::collision_event::CollisionEvent;
use crate::health::Health;
use crate::schedule::InGameSystemSet;
use crate::spaceship::{Spaceship, SpaceshipMissile};

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
        app.add_systems(Update, collision_detection.in_set(InGameSystemSet::CollisionDetection));
        app.add_systems(
            Update,
            (
                (
                    handle_collisions::<Asteroid>,
                    handle_collisions::<Spaceship>,
                    handle_collisions::<SpaceshipMissile>
                ),
                apply_collision_damage
            ).chain().in_set(InGameSystemSet::EntityUpdates),
        );
        app.add_event::<CollisionEvent>();
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

fn handle_collisions<T: Component>(
    mut collision_event_writer: EventWriter<CollisionEvent>,
    query: Query<(Entity, &Collider), With<T>>,
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.new_collisions.iter() {
            if query.get(collided_entity).is_ok() {
                continue;
            }
            collision_event_writer.send(
                CollisionEvent::new(entity, collided_entity)
            );
        }
    }
}


pub fn apply_collision_damage(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut health_query: Query<&mut Health>,
    collision_damage_query: Query<&CollisionDamage>,
) {
    for &CollisionEvent {
        entity,
        collided_entity
    } in collision_event_reader.read() {
        let Ok(mut health) = health_query.get_mut(entity) else {
            continue;
        };

        let Ok(collision_damage) = collision_damage_query.get(collided_entity) else {
            continue;
        };

        health.value -= collision_damage.value;
    }
}