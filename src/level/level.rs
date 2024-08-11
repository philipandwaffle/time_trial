use super::{blue_print::Blueprint, input::Input, logic_tree::LogicTree, output::Output};
use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Query, Resource};
use bevy_trait_query::*;

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

    pub fn update_state(
        &mut self,
        inputs: &Query<One<&dyn Input>>,
        outputs: &mut Query<One<&mut dyn Output>>,
    ) {
        let mut input_vec = vec![];
        for input_ent in self.inputs.iter() {
            if let Ok(input_component) = inputs.get(*input_ent) {
                input_component.append_state(&mut input_vec);
            }
        }

        let mut output = self.logic_tree.process(input_vec);
        for output_ent in self.outputs.iter() {
            if let Ok(mut output_component) = outputs.get_mut(*output_ent) {
                output_component.pop_state(&mut output);
            }
        }
    }
}
