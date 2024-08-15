use bevy::{
    app::{Plugin, Update},
    asset::Handle,
    prelude::{Component, Entity, Query, Res},
    sprite::ColorMaterial,
};
use bevy_rapier2d::plugin::RapierContext;
use serde::{Deserialize, Serialize};

use super::blueprints::level::LevelMaterialHandles;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn Input, ToggleButton>()
            .register_component_as::<dyn Input, PressButton>();

        app.add_systems(Update, (update_toggle_button, update_press_button));
    }
}

#[bevy_trait_query::queryable]
pub trait Input {
    fn append_state(&self, vec: &mut Vec<bool>);
    fn get_n(&self) -> usize;
}

#[derive(Deserialize, Serialize)]
pub enum InputType {
    ButtonType,
}
#[derive(Deserialize, Serialize)]
pub enum ButtonType {
    ToggleButton,
    PressButton,
}

#[derive(Component, Deserialize, Serialize)]
pub struct ToggleButton {
    state: bool,
    prev_state: bool,
    on_key: String,
    off_key: String,
}
impl ToggleButton {
    pub fn new(on_key: &str, off_key: &str) -> Self {
        return Self {
            state: false,
            prev_state: false,
            on_key: on_key.to_string(),
            off_key: off_key.to_string(),
        };
    }
}
impl Input for ToggleButton {
    fn append_state(&self, vec: &mut Vec<bool>) {
        vec.push(self.state);
    }
    fn get_n(&self) -> usize {
        return 1;
    }
}
pub fn update_toggle_button(
    rapier_context: Res<RapierContext>,
    mut buttons: Query<(Entity, &mut ToggleButton, &mut Handle<ColorMaterial>)>,
    level_material_handles: Res<LevelMaterialHandles>,
) {
    for (ent, mut button, mut color) in buttons.iter_mut() {
        let cur_collider_state = rapier_context.intersection_pairs_with(ent).count() > 0;
        if button.prev_state != cur_collider_state {
            if !button.prev_state && cur_collider_state {
                button.state = !button.state;
                match button.state {
                    true => *color = level_material_handles.0[&button.on_key].clone(),
                    false => *color = level_material_handles.0[&button.off_key].clone(),
                }
            }
            button.prev_state = cur_collider_state;
        }
    }
}

#[derive(Component, Deserialize, Serialize)]
pub struct PressButton {
    state: bool,
    on_key: String,
    off_key: String,
}
impl PressButton {
    pub fn new(on_key: &str, off_key: &str) -> Self {
        return Self {
            state: false,
            on_key: on_key.to_string(),
            off_key: off_key.to_string(),
        };
    }
}
impl Input for PressButton {
    fn append_state(&self, vec: &mut Vec<bool>) {
        vec.push(self.state);
    }
    fn get_n(&self) -> usize {
        return 1;
    }
}

pub fn update_press_button(
    rapier_context: Res<RapierContext>,
    mut buttons: Query<(Entity, &mut PressButton, &mut Handle<ColorMaterial>)>,
    level_material_handles: Res<LevelMaterialHandles>,
) {
    for (ent, mut button, mut color) in buttons.iter_mut() {
        let new_state = rapier_context.intersection_pairs_with(ent).count() > 0;
        if button.state != new_state {
            button.state = new_state;
            match button.state {
                true => *color = level_material_handles.0[&button.on_key].clone(),
                false => *color = level_material_handles.0[&button.off_key].clone(),
            }
        }
    }
}
