use bevy::{
    math::Vec2,
    prelude::{Commands, Entity},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GoalBlueprint {
    pos: Vec2,
    z_rot: f32,
    shape: Vec2,
}
impl GoalBlueprint {
    pub fn new(pos: Vec2, z_rot: f32, shape: Vec2) -> Self {
        return Self { pos, z_rot, shape };
    }
    pub fn spawn(self, commands: &mut Commands) -> Entity {
        return;
    }
}
