use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

pub trait Input {
    fn get_state(&self) -> Vec<bool>;
    fn get_n(&self) -> usize;
}

#[derive(Deserialize, Serialize)]
pub enum InputType {
    ToggleButton,
    PressButton,
}

#[derive(Component, Deserialize, Serialize)]
pub struct ToggleButton {
    pub state: bool,
}
impl Input for ToggleButton {
    fn get_state(&self) -> Vec<bool> {
        return vec![self.state];
    }
    fn get_n(&self) -> usize {
        return 1;
    }
}

#[derive(Component, Deserialize, Serialize)]
pub struct PressButton {
    pub state: bool,
}
impl Input for PressButton {
    fn get_state(&self) -> Vec<bool> {
        return vec![self.state];
    }
    fn get_n(&self) -> usize {
        return 1;
    }
}
