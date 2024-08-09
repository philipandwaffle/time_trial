use bevy::{
    asset::Handle,
    math::Vec2,
    prelude::{BuildChildren, Commands, Entity},
    sprite::{ColorMaterial, Mesh2dHandle},
};
use serde::{Deserialize, Serialize};

use crate::{configuration::ConfigTag, handles::Handles};

use super::{
    bundles::{LevelRootBundle, PressButtonBundle, ToggleButtonBundle, WallBundle},
    input::{ButtonType, InputType},
    logic_tree::LogicTree,
    output::OutputType,
};

#[derive(Deserialize, Serialize)]
pub struct Blueprint {
    pub walls: Vec<WallBluePrint>,
    pub inputs: Vec<InputBluePrint>,
    pub outputs: Vec<OutputType>,
    pub logic_tree: LogicTree,
}
impl ConfigTag for Blueprint {}
impl Blueprint {
    pub fn spawn(self, commands: &mut Commands, handles: &Handles) -> Entity {
        let root_ent = LevelRootBundle::new().spawn(commands);

        for wall in self.walls {
            let wall_ent = wall.spawn(&handles.wall_material, &handles.wall_mesh, commands);
            commands.get_entity(root_ent).unwrap().add_child(wall_ent);
        }

        for input in self.inputs {
            let input_ent = match input {
                InputBluePrint::Button(button_blue_print) => {
                    button_blue_print.spawn(commands, handles)
                }
            };
            commands.get_entity(root_ent).unwrap().add_child(input_ent);
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

#[derive(Deserialize, Serialize)]
pub enum InputBluePrint {
    Button(ButtonBluePrint),
}

#[derive(Deserialize, Serialize)]
pub struct ButtonBluePrint {
    pos: Vec2,
    radius: f32,
    button_type: ButtonType,
}
impl ButtonBluePrint {
    pub fn new(pos: Vec2, radius: f32, button_type: ButtonType) -> Self {
        return Self {
            pos,
            radius,
            button_type: button_type,
        };
    }

    pub fn spawn(self, commands: &mut Commands, handles: &Handles) -> Entity {
        return match &self.button_type {
            ButtonType::ToggleButton => ToggleButtonBundle::new(
                &handles.toggle_button_material,
                &handles.button_mesh,
                self.radius,
                self.pos,
            )
            .spawn(commands),
            ButtonType::PressButton => PressButtonBundle::new(
                &handles.press_button_material,
                &handles.button_mesh,
                self.radius,
                self.pos,
            )
            .spawn(commands),
        };
    }
}
