use core::panic;
use std::{
    fs::{self, File},
    io::{Read, Write},
};

use bevy::prelude::*;
use bevy_inspector_egui::egui::output;
use bevy_rapier2d::prelude::{Collider, RapierContext, RigidBody, Sensor};
use serde::{Deserialize, Serialize};

use self::level_components::LogicGateType;
use self::level_components::OutputType;
use self::level_components::*;
use self::level_components::{Input, InputType};

use crate::player::Player;
use crate::{config::CONFIGURATION, input::InputStates};

mod level_components;

#[derive(Resource)]
pub struct LevelContext {
    level_dirs: Vec<String>,
    cur_level_index: usize,
    cur_level: Option<Level>,
}
impl Default for LevelContext {
    fn default() -> Self {
        let mut level_dirs: Vec<String> = vec![];

        let dir_reader = match fs::read_dir(&CONFIGURATION.level.dir) {
            Ok(rd) => {
                info!("Successfully read directory: {}", &CONFIGURATION.level.dir);
                rd
            }
            Err(err) => {
                error!("Error reading directory: {}", &CONFIGURATION.level.dir);
                error!("{:?}", err);
                panic!();
            }
        };

        for dir in dir_reader {
            match dir {
                Ok(dir_entry) => {
                    let dir_path = dir_entry.path();
                    let path = dir_path.as_os_str().to_str().unwrap();
                    trace!("Found level file {}", path);
                    level_dirs.push(String::from(path));
                }
                Err(err) => {
                    error!("Error reading directory, continuing to process");
                    error!("{:?}", err);
                }
            }
        }

        if level_dirs.len() == 0 {
            warn!("No levels have been loaded");
        }

        return Self {
            level_dirs: level_dirs,
            cur_level_index: 0,
            cur_level: None,
        };
    }
}
impl LevelContext {
    pub fn change_level(&mut self, index: usize, commands: &mut Commands) {
        if index > self.level_dirs.len() - 1 {
            error!("Level with index {} doesn't exist", index);
            return;
        }

        // Despawning level
        if self.cur_level.is_some() {
            info!("Despawning current level");
            self.cur_level.as_mut().unwrap().despawn(commands);
        } else {
            info!("No current level to despawn")
        }

        // Setting new current level
        self.cur_level_index = index;

        // Spawning new level
        let path: &str = self.level_dirs.get(index).unwrap();
        self.cur_level = Level::load(path);

        if self.cur_level.is_some() {
            info!("Spawning current level");
            self.cur_level.as_mut().unwrap().spawn(commands);
        } else {
            info!("No current level to spawn");
        }
    }
    pub fn log_level_tree(&self) {
        debug!("{:?}", self.cur_level.as_ref().unwrap().tree);
    }
}
pub struct LevelControllerPlugin;
impl Plugin for LevelControllerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelContext::default())
            .add_system(level_debugging)
            .add_system(update_level_logic)
            .add_system(update_level_inputs)
            .add_system(update_level_outputs);
    }
}
fn level_debugging(mut lc: ResMut<LevelContext>, input: Res<InputStates>, mut commands: Commands) {
    let mut next_level_index = lc.cur_level_index;
    if input.next_level {
        next_level_index += 1;
    } else if input.prev_level {
        if lc.cur_level_index == 0 {
            error!("Level with index < 0 is invalid");
            return;
        } else {
            next_level_index -= 1;
        }
    } else if input.debug_level {
        info!("{:?}", lc.cur_level.as_ref().unwrap().tree);
        return;
    } else {
        return;
    }

    lc.change_level(next_level_index, &mut commands);
}
fn update_level_logic(mut lc: ResMut<LevelContext>) {
    match lc.cur_level.as_mut() {
        Some(level) => level.update_level_logic(),
        None => warn!("There is no current level"),
    }
}
fn update_level_inputs(
    rc: Res<RapierContext>,
    mut inputs: Query<(Entity, &mut Input)>,
    player: Query<(Entity, &Player)>,
) {
    let player_entity = match player.get_single() {
        Ok(p) => p.0,
        Err(_) => {
            warn!("There's no player in the level");
            return;
        }
    };

    for (input_entity, mut input) in inputs.iter_mut() {
        let intersecting = rc.intersection_pair(input_entity, player_entity) == Some(true);
        let cur_state = input.cur_state;

        match input.input_type {
            InputType::PressButton => {
                if cur_state != intersecting {
                    input.update_state(&intersecting);
                    info!("Press button new state: {}", &intersecting);
                }
            }
            InputType::ToggleButton => {
                if cur_state != intersecting {
                    input.update_state(&intersecting);
                    info!("Toggle button new state: {}", &intersecting);
                }
            }
        }
    }
}
fn update_level_outputs(mut outputs: Query<(Entity, &Output)>, mut commands: Commands) {
    for (output_entity, output) in outputs.iter_mut() {
        match output.output_type {
            OutputType::Door => {
                if output.cur_state {
                    commands.entity(output_entity).insert(Sensor);
                    debug!("Opening door");
                } else {
                    commands.entity(output_entity).remove::<Sensor>();
                    debug!("Closing door");
                }
            }
            OutputType::Light => todo!(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Level {
    walls: Vec<Wall>,
    tree: Tree,
    active_entities: Vec<Entity>,
}
impl Level {
    pub fn new(walls: Vec<Wall>, tree: Tree) -> Self {
        return Self {
            walls: walls,
            tree: tree,
            active_entities: vec![],
        };
    }

    pub fn update_level_logic(&mut self) {
        self.tree.update()
    }

    /// Spawns the level
    /// Make sure to despawn the previous level before
    pub fn spawn(&mut self, commands: &mut Commands) {
        // Clear current entity list before spawning level
        self.active_entities.clear();

        // Spawn the level and add each entity to the entity list
        self.walls.iter().for_each(|wall| {
            self.active_entities
                .push(commands.spawn(wall.build_bundle()).id());
        });

        self.tree.inputs.iter().for_each(|input| {
            self.active_entities
                .push(commands.spawn(input.build_bundle()).id());
        });

        self.tree.outputs.iter().for_each(|outputs| {
            self.active_entities
                .push(commands.spawn(outputs.build_bundle()).id());
        });
    }

    /// Despawn all entities associated with the level
    /// Make sure to call this before spawning the next level
    pub fn despawn(&mut self, commands: &mut Commands) {
        // Despawn each entity assigned to the level
        self.active_entities.iter().for_each(|entity| {
            commands.entity(*entity).despawn();
        });

        // Clear the entity list since there are all despawned
        self.active_entities.clear();
    }

    /// Load a level from a file
    pub fn load(path: &str) -> Option<Self> {
        info!("Trying to load level from: {}", path);
        // Try to open the file
        let mut file = match File::open(path) {
            Ok(file) => {
                info!("Successfully read file: {}", &path);
                file
            }
            Err(err) => {
                error!("Error reading file: {}", &path);
                error!("{:?}", err);
                return None;
            }
        };

        // Read file into string
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {
                info!("Successfully parsed file: {}", &path);
            }
            Err(err) => {
                error!("Error parsing file to string: {}", &path);
                error!("{:?}", err);
                return None;
            }
        };

        // Deserialize file contents
        let deserialized = match serde_json::from_str::<Level>(&contents) {
            Ok(res) => {
                info!("Successfully deserialized contents: {}", &contents);
                res
            }
            Err(err) => {
                error!("Error deserializing contents: {}", &contents);
                error!("{:?}", err);
                return None;
            }
        };

        return Some(deserialized);
    }

    /// Save the level data to a file
    pub fn save(&self, path: &str) {
        // Create the save file
        let mut file = match File::create(path) {
            Ok(file) => {
                info!("Creating new file: {}", &path);
                file
            }
            Err(err) => {
                error!("Error creating file: {}", &path);
                error!("{:?}", err);
                return;
            }
        };

        // Convert to JSON
        let serialized = serde_json::to_string(&self).unwrap();

        // Write to file
        match file.write_all(serialized.as_bytes()) {
            Ok(_) => info!("Wrote level to file: {}", &path),
            Err(err) => {
                error!("Didn't write to file: {}", &path);
                error!("{:?}", err);
            }
        }
    }
}

/// Sandbox to hardcode levels so they can be saved to a file
pub fn create_levels() {
    let level_01 = Level::new(
        vec![
            Wall::new(-50.0, 0.0, 0.0, 10.0, 50.0),
            Wall::new(50.0, 0.0, 0.0, 10.0, 50.0),
        ],
        Tree {
            outputs: vec![Output::new(OutputType::Door, 0, 500.0, 0.0, 50.0, 50.0)],
            gates: vec![LogicGate::new(LogicGateType::Or, [0, 1])],
            inputs: vec![
                level_components::Input::new(InputType::PressButton, 0.0, 500.0, 50.0, 50.0),
                level_components::Input::new(InputType::ToggleButton, 0.0, -500.0, 50.0, 50.0),
            ],
        },
    );

    level_01.save("levels/001.json");
}
