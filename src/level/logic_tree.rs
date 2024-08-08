use bevy::core_pipeline::core_2d::graph::input;

use super::{
    gate::{self, Gate, GateTypes},
    input::Input,
    output::Output,
};

pub struct LogicTree {
    gates: Vec<Vec<GateTypes>>,
    interfaces: Vec<Vec<usize>>,
}
impl LogicTree {
    pub fn process(&mut self, input: Vec<bool>) -> Vec<bool> {
        let first_interface = self.interfaces[0].clone();
        let mut mapped_input = Vec::with_capacity(first_interface.len());
        for i in first_interface {
            mapped_input.push(input[i]);
        }

        let mut output = vec![];
        for gate_i in 0..self.gates.len() {
            output = Vec::with_capacity(self.gates[gate_i].iter().map(|g| g.get_o()).sum());
            for gate in self.gates[gate_i].iter_mut() {
                gate.process(&mut mapped_input, &mut output);
            }

            let interface_i = gate_i + 1;
            for i in self.interfaces[interface_i].iter() {
                mapped_input.push(output[*i]);
            }
        }

        for i in self.interfaces[self.interfaces.len() - 1].iter() {
            mapped_input.push(output[*i]);
        }
        return mapped_input;
    }
}
