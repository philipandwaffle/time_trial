use std::collections::HashMap;

use bevy::{
    asset::Handle,
    math::Vec2,
    prelude::{Commands, Entity},
    sprite::{ColorMaterial, Mesh2dHandle},
};
use serde::{Deserialize, Serialize};

use crate::level::{
    bundles::input::{PressButtonBundle, ToggleButtonBundle},
    input::ButtonType,
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
    on_material_key: String,
    off_material_key: String,
}
impl ButtonBlueprint {
    pub fn new(
        pos: Vec2,
        radius: f32,
        button_type: ButtonType,
        on_material_key: &str,
        off_material_key: &str,
    ) -> Self {
        return Self {
            pos,
            radius,
            button_type: button_type,
            on_material_key: on_material_key.to_string(),
            off_material_key: off_material_key.to_string(),
        };
    }

    pub fn spawn(
        self,
        commands: &mut Commands,
        materials: &HashMap<String, Handle<ColorMaterial>>,
        mesh: &Mesh2dHandle,
    ) -> Entity {
        return match &self.button_type {
            ButtonType::ToggleButton => ToggleButtonBundle::new(
                &materials[&self.off_material_key],
                &mesh,
                self.radius,
                self.pos,
                &self.on_material_key,
                &self.off_material_key,
            )
            .spawn(commands),
            ButtonType::PressButton => PressButtonBundle::new(
                &materials[&self.off_material_key],
                &mesh,
                self.radius,
                self.pos,
                &self.on_material_key,
                &self.off_material_key,
            )
            .spawn(commands),
        };
    }
}
