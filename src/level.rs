use std::{
    fs::File,
    io::{Read, Write},
};

use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;
use serde::{Deserialize, Serialize};
pub struct LevelControllerPlugin;

#[derive(Serialize, Deserialize)]
pub struct Level {
    walls: Vec<Wall>,
    active_entities: Vec<Entity>,
}
impl Level {
    pub fn new(walls: Vec<Wall>) -> Self {
        return Self {
            walls: walls,
            active_entities: vec![],
        };
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
        // Try to open the file
        let mut file = match File::open(&path) {
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

#[derive(Serialize, Deserialize)]
pub struct Wall {
    pub pos: Vec2,
    pub rotation: f32,
    pub size: Vec2,
}
impl Wall {
    pub fn new(x_pos: f32, y_pos: f32, rotation: f32, x_size: f32, y_size: f32) -> Self {
        return Self {
            pos: Vec2::new(x_pos, y_pos),
            rotation: rotation,
            size: Vec2::new(x_size, y_size),
        };
    }
    pub fn build_bundle(&self) -> (TransformBundle, Collider) {
        return (
            TransformBundle::from_transform(Transform {
                translation: self.pos.extend(0.0),
                rotation: Quat::from_rotation_z(self.rotation),
                ..default()
            }),
            Collider::cuboid(self.size.x, self.size.y),
        );
    }
}

/// Sandbox to hardcode levels so they can be saved to a file
pub fn create_levels() {
    let level_01 = Level::new(vec![
        Wall::new(-50.0, 0.0, 0.0, 10.0, 50.0),
        Wall::new(50.0, 0.0, 0.0, 10.0, 50.0),
    ]);

    level_01.save("levels/001.json");
}
