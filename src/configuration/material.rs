use bevy::{color::Color, ecs::system::Resource, sprite::ColorMaterial};
use serde::{Deserialize, Serialize};

use super::ConfigTag;

#[derive(Resource, Serialize, Deserialize)]
pub struct MaterialConfig {
    pub wall: HSL,
    pub player: HSL,
    pub press_button: HSL,
    pub toggle_button: HSL,
    pub door: HSL,
}
impl ConfigTag for MaterialConfig {}

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
