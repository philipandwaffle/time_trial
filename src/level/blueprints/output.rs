use std::collections::HashMap;

use bevy::{
    asset::Handle,
    math::Vec2,
    prelude::{Commands, Entity},
    sprite::{ColorMaterial, Mesh2dHandle},
};
use serde::{Deserialize, Serialize};

use crate::level::bundles::output::DoorBundle;

#[derive(Deserialize, Serialize)]
pub enum OutputBluePrint {
    Door(DoorBlueprint),
}

#[derive(Deserialize, Serialize)]
pub struct DoorBlueprint {
    pos: Vec2,
    z_rot: f32,
    shape: Vec2,
    material_key: String,
}
impl DoorBlueprint {
    pub fn new(pos: Vec2, z_rot: f32, shape: Vec2, material_key: &str) -> Self {
        return Self {
            pos,
            z_rot,
            shape,
            material_key: material_key.to_string(),
        };
    }

    pub fn spawn(
        self,
        commands: &mut Commands,
        materials: &HashMap<String, Handle<ColorMaterial>>,
        mesh: &Mesh2dHandle,
    ) -> Entity {
        return DoorBundle::new(
            &materials[&self.material_key],
            mesh,
            self.pos,
            self.z_rot,
            self.shape,
        )
        .spawn(commands);
    }
}
