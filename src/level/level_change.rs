use std::fs::{self, metadata, ReadDir};

use bevy::{
    app::{Plugin, Update},
    prelude::{Commands, Event, EventReader, Res, ResMut, Resource},
};

use crate::{configuration::Config, handles::Handles};

use super::{
    blueprints::level::{LevelBlueprint, LevelMaterialHandles},
    manager::LevelManager,
};

pub struct ChangeLevelPlugin;
impl Plugin for ChangeLevelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ChangeLevelEvent>()
            .add_systems(Update, read_change_level_event);
    }
}

#[derive(Resource)]
pub struct LevelPack {
    dir: String,
    levels: Vec<String>,
    cur_i: usize,
}
impl LevelPack {
    pub fn new(dir: &str) -> Self {
        let levels = if let Ok(levels) = fs::read_dir(dir) {
            levels
                .into_iter()
                .filter(|l| l.as_ref().unwrap().metadata().unwrap().is_dir())
                .map(|l| l.unwrap().file_name().to_str().unwrap().to_string())
                .collect::<Vec<String>>()
        } else {
            vec![]
        };

        return Self {
            dir: dir.to_string(),
            levels: levels,
            cur_i: 0,
        };
    }
}

#[derive(Event)]
pub struct ChangeLevelEvent {
    path: String,
}
fn read_change_level_event(
    mut commands: Commands,
    mut ev_change_level: EventReader<ChangeLevelEvent>,
    mut level_manager: ResMut<LevelManager>,
    handles: Res<Handles>,
    level_material_handles: Res<LevelMaterialHandles>,
) {
    for ev in ev_change_level.read() {
        let blueprint = LevelBlueprint::load_cfg(&ev.path);
        level_manager.change_level(&mut commands, &handles, &level_material_handles, blueprint)
    }
}
