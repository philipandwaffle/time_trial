use bevy::math::Vec3;

pub const KEY_BINDINGS_CFG_PATH: &str = "cfg/key_bindings.json";
pub const DISPLAY_CFG_PATH: &str = "cfg/display.json";
pub const LEVEL_CFG_PATH: &str = "cfg/level.json";
pub const MATERIAL_CFG_PATH: &str = "cfg/material.json";
pub const MESH_CFG_PATH: &str = "cfg/mesh.json";
pub const PLAYER_CFG_PATH: &str = "cfg/player.json";

pub const PLAYER_Z_OFFSET: f32 = 1.0;
pub const CAMERA_Z_OFFSET: f32 = 1.0;

pub const LEVEL_Z_OFFSET: f32 = 0.0;
pub const GOAL_Z_OFFSET: f32 = 0.0;
pub const WALL_Z_OFFSET: f32 = 0.2;
pub const PROP_Z_OFFSET: f32 = 0.1;
pub const INPUT_Z_OFFSET: f32 = 0.0;
pub const OUTPUT_Z_OFFSET: f32 = 0.0;

pub const WALLS_FILE: &str = "walls.json";
pub const PLAYER_FILE: &str = "player.json";
pub const PROPS_FILE: &str = "props.json";
pub const INPUTS_FILE: &str = "inputs.json";
pub const OUTPUTS_FILE: &str = "outputs.json";
// pub const LOGIC_TREE_FILE: &str = "logic_tree.json";
pub const LOGIC_GRAPH_FILE: &str = "logic_graph.json";
pub const GOAL_FILE: &str = "goal.json";
pub const MATERIALS_FILE: &str = "materials.json";

pub const TEXT_SCALE: Vec3 = Vec3 {
    x: 0.05,
    y: 0.05,
    z: 1.0,
};
