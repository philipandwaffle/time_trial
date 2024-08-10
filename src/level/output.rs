use bevy::{app::Plugin, prelude::Component};
use serde::{Deserialize, Serialize};

pub struct OutputPlugin;
impl Plugin for OutputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        use bevy_trait_query::RegisterExt;

        app.register_component_as::<dyn Output, Door>();
    }
}

#[bevy_trait_query::queryable]
pub trait Output {
    fn get_n(&self) -> usize;
    fn update_state(&mut self, new_state: Vec<bool>);
}

#[derive(Deserialize, Serialize)]
pub enum OutputType {
    Gate,
}

#[derive(Component)]
pub struct Door {
    pub state: bool,
}
impl Output for Door {
    fn get_n(&self) -> usize {
        return 1;
    }
    fn update_state(&mut self, new_state: Vec<bool>) {
        self.state = new_state[0];
    }
}
