use serde::{Deserialize, Serialize};

use super::{input::InputType, logic_tree::LogicTree, output::OutputType};

#[derive(Deserialize, Serialize)]
pub struct BluePrint {
    logic_tree: LogicTree,
    inputs: Vec<InputType>,
    outputs: Vec<OutputType>,
}
