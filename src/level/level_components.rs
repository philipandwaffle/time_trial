use bevy::prelude::*;
use bevy_inspector_egui::egui::output;
use bevy_rapier2d::prelude::{Collider, Sensor};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Tree {
    nodes: Vec<Node>,
}
impl Tree {
    pub fn update(&mut self) {
        for node in self.nodes.iter_mut() {
            match node {
                Node::Input(_) => todo!(),
                Node::LogicGate(_) => todo!(),
                Node::Output(output) => {}
            }
        }
    }

    fn update_node(&mut self, node: Node) {}
}

#[derive(Serialize, Deserialize)]
pub enum Node {
    Input(Input),
    LogicGate(LogicGate),
    Output(Output),
}

#[derive(Serialize, Deserialize)]
pub enum InputType {
    PressButton,
    ToggleButton,
}
#[derive(Serialize, Deserialize)]
pub struct Input {
    input_type: InputType,
    cur_state: bool,
    pos: Vec2,
    size: Vec2,
}
impl Input {
    pub fn new(input_type: InputType, x_pos: f32, y_pos: f32, x_size: f32, y_size: f32) -> Self {
        return Self {
            input_type: input_type,
            cur_state: false,
            pos: Vec2::new(x_pos, y_pos),
            size: Vec2::new(x_size, y_size),
        };
    }

    pub fn build_bundle(&self) -> (TransformBundle, Collider, Sensor) {
        return (
            TransformBundle::from_transform(Transform {
                translation: self.pos.extend(0.0),
                ..default()
            }),
            Collider::cuboid(self.size.x, self.size.y),
            Sensor,
        );
    }
}

#[derive(Serialize, Deserialize)]
pub enum LogicGateType {
    Or,
    And,
}
#[derive(Serialize, Deserialize)]
pub struct LogicGate {
    logic_gate_type: LogicGateType,
    in_nodes: [usize; 2],
    input: [bool; 2],
    cur_state: bool,
}
impl LogicGate {
    pub fn new(logic_gate_type: LogicGateType, in_nodes: [usize; 2]) -> Self {
        return Self {
            logic_gate_type: logic_gate_type,
            in_nodes: in_nodes,
            input: [false, false],
            cur_state: false,
        };
    }

    pub fn update_state(&mut self, a: bool, b: bool) {
        self.cur_state = match self.logic_gate_type {
            LogicGateType::Or => self.input[0] || self.input[1],
            LogicGateType::And => self.input[0] && self.input[1],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum OutputType {
    Door,
    Light,
}
#[derive(Serialize, Deserialize)]
pub struct Output {
    output_type: OutputType,
    in_node: usize,
    cur_state: bool,
    pos: Vec2,
    size: Vec2,
}
impl Output {
    pub fn new(
        output_type: OutputType,
        in_node: usize,
        x_pos: f32,
        y_pos: f32,
        x_size: f32,
        y_size: f32,
    ) -> Self {
        return Self {
            output_type: output_type,
            in_node: in_node,
            cur_state: false,
            pos: Vec2::new(x_pos, y_pos),
            size: Vec2::new(x_size, y_size),
        };
    }

    pub fn build_bundle(&self) -> (TransformBundle, Collider) {
        return (
            TransformBundle::from_transform(Transform {
                translation: self.pos.extend(0.0),
                ..default()
            }),
            Collider::cuboid(self.size.x, self.size.y),
        );
    }
}

#[derive(Serialize, Deserialize)]
pub struct Wall {
    pub pos: Vec2,
    pub rotation: f32,
    pub size: Vec2,
}
impl Wall {
    pub fn new(x_pos: f32, y_pos: f32, rotation: f32, x_size: f32, y_size: f32) -> Self {
        return Self {
            pos: Vec2::new(x_pos, y_pos),
            rotation: rotation,
            size: Vec2::new(x_size, y_size),
        };
    }
    pub fn build_bundle(&self) -> (TransformBundle, Collider) {
        return (
            TransformBundle::from_transform(Transform {
                translation: self.pos.extend(0.0),
                rotation: Quat::from_rotation_z(self.rotation),
                ..default()
            }),
            Collider::cuboid(self.size.x, self.size.y),
        );
    }
}
