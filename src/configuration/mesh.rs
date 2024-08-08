use bevy::{ecs::system::Resource, math::Vec2};
use serde::{Deserialize, Serialize};

use super::ConfigTag;

#[derive(Resource, Serialize, Deserialize)]
pub struct MeshConfig {
    pub wall: Vec2,
    pub player: f32,
}
impl ConfigTag for MeshConfig {}
