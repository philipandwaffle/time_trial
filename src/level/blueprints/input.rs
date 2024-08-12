use bevy::{
    math::Vec2,
    prelude::{Commands, Entity},
};
use serde::{Deserialize, Serialize};

use crate::{
    handles::Handles,
    level::{
        bundles::input::{PressButtonBundle, ToggleButtonBundle},
        input::ButtonType,
    },
};

#[derive(Deserialize, Serialize)]
pub enum InputBlueprint {
    Button(ButtonBlueprint),
}

#[derive(Deserialize, Serialize)]
pub struct ButtonBlueprint {
    pos: Vec2,
    radius: f32,
    button_type: ButtonType,
}
impl ButtonBlueprint {
    pub fn new(pos: Vec2, radius: f32, button_type: ButtonType) -> Self {
        return Self {
            pos,
            radius,
            button_type: button_type,
        };
    }

    pub fn spawn(self, commands: &mut Commands, handles: &Handles) -> Entity {
        return match &self.button_type {
            ButtonType::ToggleButton => ToggleButtonBundle::new(
                &handles.toggle_button_material,
                &handles.button_mesh,
                self.radius,
                self.pos,
            )
            .spawn(commands),
            ButtonType::PressButton => PressButtonBundle::new(
                &handles.press_button_material,
                &handles.button_mesh,
                self.radius,
                self.pos,
            )
            .spawn(commands),
        };
    }
}
