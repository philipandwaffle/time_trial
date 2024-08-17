use std::{
    fs::{self},
    usize,
};

use bevy::{
    app::{Plugin, PreStartup, Update},
    asset::Assets,
    input::ButtonInput,
    prelude::{Commands, Event, EventReader, EventWriter, KeyCode, Res, ResMut, Resource},
    sprite::ColorMaterial,
};

use crate::{
    configuration::{key_bindings::KeyBinds, level::LevelConfig, Config},
    handles::Handles,
};

use super::{
    blueprints::level::{LevelBlueprint, LevelMaterialHandles},
    manager::LevelManager,
    time_shift::TimeState,
};

pub struct LevelPackPlugin;
impl Plugin for LevelPackPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ChangeLevelEvent>()
            .add_systems(PreStartup, setup_default_pack)
            .add_systems(Update, (read_change_level_event, write_change_level_event));
    }
}
pub fn setup_default_pack(mut commands: Commands, level_config: Res<LevelConfig>) {
    commands.insert_resource(LevelPack::new(&level_config.dir));
}

#[derive(Resource)]
pub struct LevelPack {
    dir: String,
    levels: Vec<String>,
    cur_i: isize,
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

    pub fn change_level(&mut self, delta: isize) -> LevelBlueprint {
        self.set_cur_i(delta);

        return LevelBlueprint::load_cfg(&format!(
            "{}/{}",
            self.dir, self.levels[self.cur_i as usize]
        ));
    }

    fn set_cur_i(&mut self, delta: isize) {
        self.cur_i += delta;
        let num_levels = self.levels.len() as isize;
        if self.cur_i == num_levels {
            self.cur_i = 0;
        } else if self.cur_i == -1 {
            self.cur_i = num_levels - 1;
        }
    }
}

#[derive(Event)]
pub struct ChangeLevelEvent {
    delta: isize,
}
impl ChangeLevelEvent {
    pub fn new(delta: isize) -> Self {
        return Self { delta };
    }
}
fn read_change_level_event(
    mut commands: Commands,
    mut ev_change_level: EventReader<ChangeLevelEvent>,
    mut time_state: ResMut<TimeState>,
    mut level_manager: ResMut<LevelManager>,
    mut level_pack: ResMut<LevelPack>,
    mut level_material_handles: ResMut<LevelMaterialHandles>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    handles: Res<Handles>,
) {
    for ev in ev_change_level.read() {
        let blueprint = level_pack.change_level(ev.delta);
        level_manager.change_level(
            blueprint,
            &mut commands,
            &handles,
            &mut level_material_handles,
            &mut materials,
        );
        time_state.is_present = !time_state.is_present;
    }
}

fn write_change_level_event(
    mut ev_change_level: EventWriter<ChangeLevelEvent>,
    keys: Res<ButtonInput<KeyCode>>,
    key_binds: Res<KeyBinds>,
) {
    if keys.just_pressed(key_binds.next_level.0) {
        ev_change_level.send(ChangeLevelEvent::new(1));
    } else if keys.just_pressed(key_binds.prev_level.0) {
        ev_change_level.send(ChangeLevelEvent::new(-1));
    }
}