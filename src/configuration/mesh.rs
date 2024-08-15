use bevy::ecs::system::Resource;
use serde::{Deserialize, Serialize};

use super::ConfigTag;

#[derive(Resource, Serialize, Deserialize)]
pub struct MeshConfig {
    pub player: f32,
}
impl ConfigTag for MeshConfig {}
