use bevy::{color::Color, ecs::system::Resource, sprite::ColorMaterial};
use serde::{Deserialize, Serialize};

use super::ConfigTag;
#[derive(Resource, Serialize, Deserialize)]
pub struct HSL {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}
impl HSL {
    pub fn to_color_mat(&self) -> ColorMaterial {
        return ColorMaterial::from(Color::hsl(self.h, self.s, self.l));
    }
}

#[derive(Resource, Serialize, Deserialize)]
pub struct MaterialConfig {
    pub block: HSL,
    pub joint: HSL,
    pub bone: HSL,
    pub muscle_contract: HSL,
    pub muscle_expand: HSL,
    pub muscle_neutral: HSL,
}
impl ConfigTag for MaterialConfig {}
