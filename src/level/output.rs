use bevy::{
    app::{Plugin, Update},
    prelude::{Changed, Commands, Component, Entity, Query},
};
use bevy_rapier2d::prelude::Sensor;
use bevy_trait_query::RegisterExt;
use serde::{Deserialize, Serialize};

pub struct OutputPlugin;
impl Plugin for OutputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_component_as::<dyn Output, Door>();

        app.add_systems(Update, update_door);
    }
}

#[bevy_trait_query::queryable]
pub trait Output {
    fn get_n(&self) -> usize;
    fn needs_state_update(&self, new_state: &mut Vec<bool>) -> bool;
    fn pop_state(&mut self, new_state: &mut Vec<bool>);
}

#[derive(Deserialize, Serialize)]
pub enum OutputType {
    Door,
}

#[derive(Component)]
pub struct Door {
    pub state: bool,
}
impl Default for Door {
    fn default() -> Self {
        Self { state: true }
    }
}
impl Output for Door {
    fn get_n(&self) -> usize {
        return 1;
    }
    fn needs_state_update(&self, new_state: &mut Vec<bool>) -> bool {
        let needs_update = self.state != *new_state.last().unwrap();
        if !needs_update {
            new_state.pop();
        }
        return needs_update;
    }
    fn pop_state(&mut self, new_state: &mut Vec<bool>) {
        self.state = new_state.pop().unwrap();
    }
}
pub fn update_door(mut commands: Commands, mut doors: Query<(Entity, &Door), Changed<Door>>) {
    for (ent, door) in doors.iter_mut() {
        if let Some(mut ent_commands) = commands.get_entity(ent) {
            if door.state {
                ent_commands.insert(Sensor);
            } else {
                ent_commands.remove::<Sensor>();
            }
        }
    }
}
