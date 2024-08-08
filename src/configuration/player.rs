use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use super::ConfigTag;

#[derive(Resource, Serialize, Deserialize)]
pub struct PlayerConfig {
    pub collider_radius: f32,
    pub cam_scale: f32,
    pub move_speed: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
}
impl ConfigTag for PlayerConfig {}
