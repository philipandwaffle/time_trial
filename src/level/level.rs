use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Query, Resource};

use super::{blue_print::Blueprint, input::Input, logic_tree::LogicTree, output::Output};

#[derive(Resource)]
pub struct Level {
    root: Entity,
    logic_tree: LogicTree,
    inputs: Vec<Entity>,
    outputs: Vec<Entity>,
}
impl Level {
    pub fn new(
        root: Entity,
        logic_tree: LogicTree,
        inputs: Vec<Entity>,
        outputs: Vec<Entity>,
    ) -> Self {
        return Self {
            root,
            logic_tree,
            inputs,
            outputs,
        };
    }

    pub fn despawn(&self, commands: &mut Commands) {
        commands.entity(self.root).despawn_recursive();
    }

    pub fn update_state(&mut self, inputs: Query<&dyn Input>, outputs: Query<&dyn Output>) {
        let input = vec![];
        for input in self.inputs {
            let foo = inputs.get(input).unwrap();
            foo.get_state()
        }

        self.logic_tree.process(input)
    }
}
