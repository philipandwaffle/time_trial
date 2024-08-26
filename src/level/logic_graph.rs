use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LogicGraph {
    input_nodes: Vec<usize>,
    output_nodes: Vec<usize>,
    nodes: Vec<Node>,
}
impl LogicGraph {
    pub fn new(input_nodes: Vec<usize>, output_nodes: Vec<usize>, nodes: Vec<Node>) -> Self {
        return Self {
            input_nodes,
            output_nodes,
            nodes,
        };
    }

    pub fn has_no_logic(&self) -> bool {
        return self.nodes.is_empty();
    }

    pub fn process(&mut self, input: Vec<bool>) -> Vec<bool> {
        if input.len() != self.input_nodes.len() {
            panic!();
        }
        for i in 0..input.len() {
            self.nodes[self.input_nodes[i]].state = input[i];
        }

        let num_out = self.output_nodes.len();
        let mut output = Vec::with_capacity(num_out);
        for i in 0..num_out {
            let in_i = self.output_nodes[i];
            self.update_node(in_i);
            output.push(self.nodes[in_i].state);
        }

        self.reset_updated();
        return output;
    }

    fn update_node(&mut self, i: usize) {
        let children = self.nodes[i].in_nodes.clone();

        for child_i in children.iter() {
            self.update_node(*child_i);
        }

        if self.nodes[i].updated {
            return;
        }

        self.nodes[i].state = match self.nodes[i].op {
            Operator::None => return,
            Operator::And => {
                let mut s = true;
                for child_i in children {
                    s = s && self.nodes[child_i].state;
                }
                s
            }
            Operator::Or => {
                let mut s = false;
                for child_i in children {
                    s = s || self.nodes[child_i].state;
                }
                s
            }
            Operator::Not => {
                let mut s = false;
                for child_i in children {
                    s = s || self.nodes[child_i].state;
                }
                !s
            }
        };

        self.nodes[i].updated = true;
    }

    fn reset_updated(&mut self) {
        for node in self.nodes.iter_mut() {
            node.updated = false;
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Node {
    #[serde(skip)]
    state: bool,
    #[serde(skip)]
    updated: bool,
    op: Operator,
    in_nodes: Vec<usize>,
}
impl Node {
    pub fn new(op: Operator, in_nodes: Vec<usize>) -> Self {
        return Self {
            state: false,
            updated: false,
            op,
            in_nodes,
        };
    }
}

#[derive(Deserialize, Serialize)]
pub enum Operator {
    None,
    And,
    Or,
    Not,
}
