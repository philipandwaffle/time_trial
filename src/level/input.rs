use bevy::{app::Plugin, prelude::Component};
use serde::{Deserialize, Serialize};

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn Input, ToggleButton>()
            .register_component_as::<dyn Input, PressButton>();
    }
}
#[bevy_trait_query::queryable]
pub trait Input {
    fn get_state(&self) -> Vec<bool>;
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
}
impl Input for ToggleButton {
    fn get_state(&self) -> Vec<bool> {
        return vec![self.state];
    }
    fn get_n(&self) -> usize {
        return 1;
    }
}
impl Default for ToggleButton {
    fn default() -> Self {
        Self { state: false }
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
impl Default for PressButton {
    fn default() -> Self {
        Self { state: false }
    }
}
