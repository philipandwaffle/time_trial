use bevy::{color::Color, ecs::system::Resource, sprite::ColorMaterial};
use serde::{Deserialize, Serialize};

use super::ConfigTag;

#[derive(Resource, Serialize, Deserialize)]
pub struct MaterialConfig {
    pub player: HSL,
}
impl ConfigTag for MaterialConfig {}

#[derive(Resource, Serialize, Deserialize)]
pub struct HSL {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}
impl HSL {
    pub fn new(h: f32, s: f32, l: f32) -> Self {
        return Self { h, s, l };
    }
    pub fn to_color_mat(&self) -> ColorMaterial {
        return ColorMaterial::from(Color::hsl(self.h, self.s, self.l));
    }
}
