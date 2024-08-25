use std::{collections::HashMap, fs::create_dir, path::Path};

use bevy::{
    asset::{Assets, Handle},
    log::error,
    math::{Vec2, Vec3},
    prelude::{BuildChildren, Commands, Resource},
    sprite::ColorMaterial,
};
use serde::{Deserialize, Serialize};

use crate::{
    configuration::{material::HSL, Config, ConfigTag},
    consts::{
        GOAL_FILE, INPUTS_FILE, LOGIC_GRAPH_FILE, MATERIALS_FILE, OUTPUTS_FILE, PLAYER_FILE,
        PLAYER_Z_OFFSET, PROPS_FILE, WALLS_FILE,
    },
    handles::Handles,
    level::{bundles::level::LevelRootBundle, level::Level, logic_graph::LogicGraph},
};

use super::{
    goal::GoalBlueprint, input::InputBlueprint, output::OutputBluePrint, props::PropBlueprint,
    wall::WallBluePrint,
};

pub struct LevelBlueprint {
    player: PlayerBlueprint,
    walls: WallBlueprints,
    props: PropBlueprints,
    inputs: InputBlueprints,
    outputs: OutputBlueprints,
    goal: GoalBlueprint,
    pub logic_graph: LogicGraph,
    level_materials: LevelMaterials,
}
impl Config for LevelBlueprint {
    fn load_cfg(path: &str) -> Self {
        return Self {
            player: PlayerBlueprint::load_cfg(&format!("{}/{}", path, PLAYER_FILE)),
            walls: WallBlueprints::load_cfg(&format!("{}/{}", path, WALLS_FILE)),
            props: PropBlueprints::load_cfg(&format!("{}/{}", path, PROPS_FILE)),
            inputs: InputBlueprints::load_cfg(&format!("{}/{}", path, INPUTS_FILE)),
            outputs: OutputBlueprints::load_cfg(&format!("{}/{}", path, OUTPUTS_FILE)),
            goal: GoalBlueprint::load_cfg(&format!("{}/{}", path, GOAL_FILE)),
            logic_graph: LogicGraph::load_cfg(&format!("{}/{}", path, LOGIC_GRAPH_FILE)),
            level_materials: LevelMaterials::load_cfg(&format!("{}/{}", path, MATERIALS_FILE)),
        };
    }

    fn save_cfg(&self, path: &str) {
        if !Path::new(path).exists() {
            if let Err(err) = create_dir(path) {
                error!("Error creating dir {path}, {err}");
            }
        }

        self.player.save_cfg(&format!("{}/{}", path, PLAYER_FILE));
        self.walls.save_cfg(&format!("{}/{}", path, WALLS_FILE));
        self.props.save_cfg(&format!("{}/{}", path, PROPS_FILE));
        self.inputs.save_cfg(&format!("{}/{}", path, INPUTS_FILE));
        self.outputs.save_cfg(&format!("{}/{}", path, OUTPUTS_FILE));
        self.goal.save_cfg(&format!("{}/{}", path, GOAL_FILE));
        self.logic_graph
            .save_cfg(&format!("{}/{}", path, LOGIC_GRAPH_FILE));
        self.level_materials
            .save_cfg(&format!("{}/{}", path, MATERIALS_FILE));
    }
}
impl LevelBlueprint {
    pub fn new(
        player: Vec2,
        walls: Vec<WallBluePrint>,
        props: Vec<PropBlueprint>,
        inputs: Vec<InputBlueprint>,
        outputs: Vec<OutputBluePrint>,
        goal: GoalBlueprint,
        logic_graph: LogicGraph,
        level_materials: HashMap<String, HSL>,
    ) -> Self {
        return Self {
            player: PlayerBlueprint(player),
            walls: WallBlueprints(walls),
            props: PropBlueprints(props),
            inputs: InputBlueprints(inputs),
            outputs: OutputBlueprints(outputs),
            goal: goal,
            logic_graph,
            level_materials: LevelMaterials(level_materials),
        };
    }

    pub fn spawn(
        self,
        commands: &mut Commands,
        player_pos: &mut Vec3,
        handles: &Handles,
        level_material_handles: &mut LevelMaterialHandles,
        materials: &mut Assets<ColorMaterial>,
    ) -> Level {
        self.setup_level_material_handles(level_material_handles, materials);
        let materials = &level_material_handles.0;
        let root = LevelRootBundle::new().spawn(commands);
        *player_pos = self.player.0.extend(PLAYER_Z_OFFSET);

        for wall in self.walls.0 {
            let wall_ent = wall.spawn(materials, &handles.square_mesh, commands);
            commands.get_entity(root).unwrap().add_child(wall_ent);
        }
        for prop in self.props.0 {
            let prop_ent = match prop {
                PropBlueprint::BoxBlueprint(box_blueprint) => {
                    box_blueprint.spawn(commands, materials, &handles.square_mesh)
                }
            };
            commands.get_entity(root).unwrap().add_child(prop_ent);
        }

        let mut input_ents = Vec::with_capacity(self.inputs.0.len());
        for input in self.inputs.0 {
            let input_ent = match input {
                InputBlueprint::Button(button_blueprint) => {
                    button_blueprint.spawn(commands, materials, &handles.circle_mesh)
                }
            };
            commands.get_entity(root).unwrap().add_child(input_ent);
            input_ents.push(input_ent);
        }

        let mut output_ents = Vec::with_capacity(self.outputs.0.len());
        for output in self.outputs.0 {
            let output_ent = match output {
                OutputBluePrint::Door(door) => {
                    door.spawn(commands, materials, &handles.square_mesh)
                }
            };
            commands.get_entity(root).unwrap().add_child(output_ent);
            output_ents.push(output_ent)
        }

        let goal_ent = self.goal.spawn(commands, materials, &handles.square_mesh);
        commands.get_entity(root).unwrap().add_child(goal_ent);

        return Level::new(root, self.logic_graph, input_ents, output_ents);
    }

    pub fn setup_level_material_handles(
        &self,
        level_material_handles: &mut LevelMaterialHandles,
        materials: &mut Assets<ColorMaterial>,
    ) {
        level_material_handles.0.clear();
        for (key, color) in self.level_materials.0.iter() {
            let color_handle = materials.add(color.to_color_mat());
            level_material_handles.0.insert(key.clone(), color_handle);
        }
    }
}

#[derive(Deserialize, Serialize)]
struct PlayerBlueprint(Vec2);
impl ConfigTag for PlayerBlueprint {}

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

impl ConfigTag for LogicGraph {}

#[derive(Deserialize, Serialize)]
pub struct LevelMaterials(HashMap<String, HSL>);
impl ConfigTag for LevelMaterials {}

#[derive(Resource, Default)]
pub struct LevelMaterialHandles(pub HashMap<String, Handle<ColorMaterial>>);
