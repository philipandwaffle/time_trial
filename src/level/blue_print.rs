use bevy::{
    asset::Handle,
    math::Vec2,
    prelude::{BuildChildren, Commands, Entity},
    sprite::{ColorMaterial, Mesh2dHandle},
};
use serde::{Deserialize, Serialize};

use crate::{configuration::ConfigTag, handles::Handles};

use super::{
    bundles::{LevelRootBundle, WallBundle},
    input::InputType,
    logic_tree::LogicTree,
    output::OutputType,
};

#[derive(Deserialize, Serialize)]
pub struct BluePrint {
    pub logic_tree: LogicTree,
    pub inputs: Vec<InputType>,
    pub outputs: Vec<OutputType>,
    pub walls: Vec<WallBluePrint>,
}
impl ConfigTag for BluePrint {}
impl BluePrint {
    pub fn spawn(self, commands: &mut Commands, handles: &Handles) -> Entity {
        let root_ent = LevelRootBundle::new().spawn(commands);

        for wall in self.walls {
            let wall_ent = wall.spawn(&handles.wall_material, &handles.wall_mesh, commands);
            commands.get_entity(root_ent).unwrap().add_child(wall_ent);
        }

        return root_ent;
    }
}

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
