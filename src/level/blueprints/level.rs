use std::{collections::HashMap, fs::create_dir, path::Path};

use bevy::{
    log::error,
    prelude::{BuildChildren, Commands},
};
use serde::{Deserialize, Serialize};

use crate::{
    configuration::{material::HSL, Config, ConfigTag},
    consts::{INPUTS_FILE, LOGIC_TREE_FILE, MATERIALS_FILE, OUTPUTS_FILE, PROPS_FILE, WALLS_FILE},
    handles::Handles,
    level::{bundles::level::LevelRootBundle, level::Level, logic_tree::LogicTree},
};

use super::{
    input::InputBlueprint, output::OutputBluePrint, props::PropBlueprint, wall::WallBluePrint,
};

pub struct LevelBlueprint {
    walls: WallBlueprints,
    props: PropBlueprints,
    inputs: InputBlueprints,
    outputs: OutputBlueprints,
    pub logic_tree: LogicTree,
    level_materials: LevelMaterials,
}
impl Config for LevelBlueprint {
    fn load_cfg(path: &str) -> Self {
        return Self {
            walls: WallBlueprints::load_cfg(&format!("{}/{}", path, WALLS_FILE)),
            props: PropBlueprints::load_cfg(&format!("{}/{}", path, PROPS_FILE)),
            inputs: InputBlueprints::load_cfg(&format!("{}/{}", path, INPUTS_FILE)),
            outputs: OutputBlueprints::load_cfg(&format!("{}/{}", path, OUTPUTS_FILE)),
            logic_tree: LogicTree::load_cfg(&format!("{}/{}", path, LOGIC_TREE_FILE)),
            level_materials: LevelMaterials::load_cfg(&format!("{}/{}", path, MATERIALS_FILE)),
        };
    }

    fn save_cfg(&self, path: &str) {
        if !Path::new(path).exists() {
            if let Err(err) = create_dir(path) {
                error!("Error creating dir {path}, {err}");
            }
        }

        self.walls.save_cfg(&format!("{}/{}", path, WALLS_FILE));
        self.props.save_cfg(&format!("{}/{}", path, PROPS_FILE));
        self.inputs.save_cfg(&format!("{}/{}", path, INPUTS_FILE));
        self.outputs.save_cfg(&format!("{}/{}", path, OUTPUTS_FILE));
        self.logic_tree
            .save_cfg(&format!("{}/{}", path, LOGIC_TREE_FILE));
    }
}
impl LevelBlueprint {
    pub fn new(
        walls: Vec<WallBluePrint>,
        props: Vec<PropBlueprint>,
        inputs: Vec<InputBlueprint>,
        outputs: Vec<OutputBluePrint>,
        logic_tree: LogicTree,
        level_materials: HashMap<String, HSL>,
    ) -> Self {
        return Self {
            walls: WallBlueprints(walls),
            props: PropBlueprints(props),
            inputs: InputBlueprints(inputs),
            outputs: OutputBlueprints(outputs),
            logic_tree,
            level_materials: LevelMaterials(level_materials),
        };
    }

    pub fn spawn(self, commands: &mut Commands, handles: &Handles) -> Level {
        let root = LevelRootBundle::new().spawn(commands);

        for wall in self.walls.0 {
            let wall_ent = wall.spawn(&handles.wall_material, &handles.wall_mesh, commands);
            commands.get_entity(root).unwrap().add_child(wall_ent);
        }
        for prop in self.props.0 {
            let prop_ent = match prop {
                PropBlueprint::BoxBlueprint(box_blueprint) => {
                    box_blueprint.spawn(commands, handles)
                }
            };
            commands.get_entity(root).unwrap().add_child(prop_ent);
        }

        let mut input_ents = Vec::with_capacity(self.inputs.0.len());
        for input in self.inputs.0 {
            let input_ent = match input {
                InputBlueprint::Button(button_blueprint) => {
                    button_blueprint.spawn(commands, handles)
                }
            };
            commands.get_entity(root).unwrap().add_child(input_ent);
            input_ents.push(input_ent);
        }

        let mut output_ents = Vec::with_capacity(self.outputs.0.len());
        for output in self.outputs.0 {
            let output_ent = match output {
                OutputBluePrint::Door(door) => door.spawn(commands, handles),
            };
            commands.get_entity(root).unwrap().add_child(output_ent);
            output_ents.push(output_ent)
        }

        return Level::new(root, self.logic_tree, input_ents, output_ents);
    }
}

#[derive(Deserialize, Serialize)]
struct WallBlueprints(Vec<WallBluePrint>);
impl ConfigTag for WallBlueprints {}

#[derive(Deserialize, Serialize)]
struct PropBlueprints(Vec<PropBlueprint>);
impl ConfigTag for PropBlueprints {}

#[derive(Deserialize, Serialize)]
struct InputBlueprints(Vec<InputBlueprint>);
impl ConfigTag for InputBlueprints {}

#[derive(Deserialize, Serialize)]
struct OutputBlueprints(Vec<OutputBluePrint>);
impl ConfigTag for OutputBlueprints {}

impl ConfigTag for LogicTree {}

#[derive(Deserialize, Serialize)]
pub struct LevelMaterials(HashMap<String, HSL>);
impl ConfigTag for LevelMaterials {}
