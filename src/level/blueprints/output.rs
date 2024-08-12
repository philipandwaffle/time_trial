use bevy::{
    math::Vec2,
    prelude::{Commands, Entity},
};
use serde::{Deserialize, Serialize};

use crate::{handles::Handles, level::bundles::output::DoorBundle};

#[derive(Deserialize, Serialize)]
pub enum OutputBluePrint {
    Door(DoorBlueprint),
}

#[derive(Deserialize, Serialize)]
pub struct DoorBlueprint {
    pos: Vec2,
    z_rot: f32,
    shape: Vec2,
}
impl DoorBlueprint {
    pub fn new(pos: Vec2, z_rot: f32, shape: Vec2) -> Self {
        return Self { pos, z_rot, shape };
    }

    pub fn spawn(self, commands: &mut Commands, handles: &Handles) -> Entity {
        return DoorBundle::new(
            &handles.door_material,
            &handles.door_mesh,
            self.pos,
            self.z_rot,
            self.shape,
        )
        .spawn(commands);
    }
}
