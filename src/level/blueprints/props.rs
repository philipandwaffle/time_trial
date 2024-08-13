use bevy::{
    math::Vec2,
    prelude::{Commands, Entity},
};
use serde::{Deserialize, Serialize};

use crate::{handles::Handles, level::bundles::props::BoxBundle};

#[derive(Deserialize, Serialize)]
pub enum PropBlueprint {
    BoxBlueprint(BoxBlueprint),
}

#[derive(Deserialize, Serialize)]
pub struct BoxBlueprint {
    pos: Vec2,
    z_rot: f32,
    shape: Vec2,
}
impl BoxBlueprint {
    pub fn new(pos: Vec2, z_rot: f32, shape: Vec2) -> Self {
        return Self { pos, z_rot, shape };
    }

    pub fn spawn(self, commands: &mut Commands, handles: &Handles) -> Entity {
        return BoxBundle::new(
            &handles.wall_material,
            &handles.wall_mesh,
            self.shape,
            self.pos,
            self.z_rot,
            10.0,
            1.0,
        )
        .spawn(commands);
    }
}
