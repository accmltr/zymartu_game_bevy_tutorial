use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct CollisionDamage {
    pub value: f32,
}

impl CollisionDamage {
    pub fn new(value: f32) -> Self {
        Self {
            value
        }
    }
}