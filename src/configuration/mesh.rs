use bevy::{ecs::system::Resource, math::Vec2};
use serde::{Deserialize, Serialize};

use super::ConfigTag;

#[derive(Resource, Serialize, Deserialize)]
pub struct MeshConfig {
    pub block: Vec2,
    pub joint: f32,
    pub bone: Vec2,
    pub muscle: Vec2,
}
impl ConfigTag for MeshConfig {}
