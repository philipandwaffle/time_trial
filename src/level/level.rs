use bevy::prelude::{Entity, Resource};

use super::logic_tree::LogicTree;

#[derive(Resource)]
pub struct Level {
    inputs: Vec<Entity>,
    logic_tree: LogicTree,
    outputs: Vec<Entity>,
}
