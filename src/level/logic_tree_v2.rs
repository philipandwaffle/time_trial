pub struct LogicGraph {
    input_nodes: Vec<usize>,
    output_nodes: Vec<usize>,
    nodes: Vec<Node>,
}
impl LogicGraph {
    pub fn process(&mut self, input: Vec<bool>) -> Vec<bool> {
        if input.len() != self.output_nodes.len() {
            panic!();
        }
        for i in 0..input.len() {
            self.nodes[self.input_nodes[i]].state = input[i];
        }

        return vec![];
    }

    fn update_node(&mut self, i: usize) {
        let children = self.nodes[i].in_nodes.clone();
        for child_i in children {
            self.update_node(child_i);
        }
    }
}

pub struct Node {
    state: bool,
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

enum Operator {
    And,
    Or,
}

struct Wire {
    in_indices: Vec<usize>,
    out_indices: Vec<usize>,
}
