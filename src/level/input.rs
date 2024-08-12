use bevy::{
    app::{Plugin, Update},
    prelude::{Component, Entity, Query, Res},
};
use bevy_rapier2d::plugin::RapierContext;
use serde::{Deserialize, Serialize};

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
    pub state: bool,
    prev_state: bool,
}
impl Input for ToggleButton {
    fn append_state(&self, vec: &mut Vec<bool>) {
        vec.push(self.state);
    }
    fn get_n(&self) -> usize {
        return 1;
    }
}
impl Default for ToggleButton {
    fn default() -> Self {
        Self {
            state: false,
            prev_state: false,
        }
    }
}
pub fn update_toggle_button(
    rapier_context: Res<RapierContext>,
    mut buttons: Query<(Entity, &mut ToggleButton)>,
) {
    for (ent, mut button) in buttons.iter_mut() {
        let cur_collider_state = rapier_context.intersection_pairs_with(ent).count() > 0;
        if button.prev_state != cur_collider_state {
            if !button.prev_state && cur_collider_state {
                button.state = !button.state;
            }
            button.prev_state = cur_collider_state;
        }
    }
}

#[derive(Component, Deserialize, Serialize)]
pub struct PressButton {
    pub state: bool,
}
impl Input for PressButton {
    fn append_state(&self, vec: &mut Vec<bool>) {
        vec.push(self.state);
    }
    fn get_n(&self) -> usize {
        return 1;
    }
}
impl Default for PressButton {
    fn default() -> Self {
        Self { state: false }
    }
}
pub fn update_press_button(
    rapier_context: Res<RapierContext>,
    mut buttons: Query<(Entity, &mut PressButton)>,
) {
    for (ent, mut button) in buttons.iter_mut() {
        button.state = rapier_context.intersection_pairs_with(ent).count() > 0;
    }
}
