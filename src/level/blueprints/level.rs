use bevy::prelude::{BuildChildren, Commands};
use serde::{Deserialize, Serialize};

use crate::{
    configuration::ConfigTag,
    handles::Handles,
    level::{bundles::level::LevelRootBundle, level::Level, logic_tree::LogicTree},
};

use super::{
    input::InputBlueprint, output::OutputBluePrint, props::PropBlueprint, wall::WallBluePrint,
};

#[derive(Deserialize, Serialize)]
pub struct Blueprint {
    pub walls: Vec<WallBluePrint>,
    pub props: Vec<PropBlueprint>,
    pub inputs: Vec<InputBlueprint>,
    pub outputs: Vec<OutputBluePrint>,
    pub logic_tree: LogicTree,
}
impl ConfigTag for Blueprint {}
impl Blueprint {
    pub fn spawn(self, commands: &mut Commands, handles: &Handles) -> Level {
        let root = LevelRootBundle::new().spawn(commands);

        for wall in self.walls {
            let wall_ent = wall.spawn(&handles.wall_material, &handles.wall_mesh, commands);
            commands.get_entity(root).unwrap().add_child(wall_ent);
        }
        for prop in self.props {
            let prop_ent = match prop {
                PropBlueprint::BoxBlueprint(box_blueprint) => {
                    box_blueprint.spawn(commands, handles)
                }
            };
            commands.get_entity(root).unwrap().add_child(prop_ent);
        }

        let mut input_ents = Vec::with_capacity(self.inputs.len());
        for input in self.inputs {
            let input_ent = match input {
                InputBlueprint::Button(button_blueprint) => {
                    button_blueprint.spawn(commands, handles)
                }
            };
            commands.get_entity(root).unwrap().add_child(input_ent);
            input_ents.push(input_ent);
        }

        let mut output_ents = Vec::with_capacity(self.outputs.len());
        for output in self.outputs {
            let output_ent = match output {
                OutputBluePrint::Door(door) => door.spawn(commands, handles),
            };
            output_ents.push(output_ent)
        }

        return Level::new(root, self.logic_tree, input_ents, output_ents);
    }
}
