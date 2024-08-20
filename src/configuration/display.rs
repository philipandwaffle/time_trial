use bevy::math::{IVec2, Vec2};
use serde::{Deserialize, Serialize};

use super::ConfigTag;

#[derive(Deserialize, Serialize)]
pub struct DisplayConfig {
    pub pos: IVec2,
    pub resolution: Vec2,
}
impl ConfigTag for DisplayConfig {}
