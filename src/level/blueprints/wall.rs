use bevy::{
    asset::Handle,
    math::Vec2,
    prelude::{Commands, Entity},
    sprite::{ColorMaterial, Mesh2dHandle},
};
use serde::{Deserialize, Serialize};

use crate::level::bundles::wall::WallBundle;

#[derive(Deserialize, Serialize)]
pub struct WallBluePrint {
    pos: Vec2,
    z_rot: f32,
    shape: Vec2,
}
impl WallBluePrint {
    pub fn new(pos: Vec2, z_rot: f32, shape: Vec2) -> Self {
        return Self { pos, z_rot, shape };
    }

    pub fn spawn(
        self,
        material: &Handle<ColorMaterial>,
        mesh: &Mesh2dHandle,
        commands: &mut Commands,
    ) -> Entity {
        return WallBundle::new(material, mesh, self.pos, self.z_rot, self.shape).spawn(commands);
    }
}
