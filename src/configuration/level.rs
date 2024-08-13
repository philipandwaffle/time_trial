use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

use super::ConfigTag;

#[derive(Resource, Deserialize, Serialize)]
pub struct LevelConfig {
    pub dir: String,
    pub cur_level: String,
    pub gen_on_start: bool,
    pub save_on_start: bool,
}
impl ConfigTag for LevelConfig {}
