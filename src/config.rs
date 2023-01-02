use std::fs;

use bevy::prelude::Plugin;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub logging: Logging,
    pub level: Level,
    pub player: Player,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Logging {
    pub level: String,
    pub filter: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Level {
    pub dir: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub move_speed: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
}

lazy_static! {
    pub static ref CONFIGURATION: Config = {
        let config_path: &str = "cfg.json";
        let config: &str =
            &fs::read_to_string(config_path).expect("Something went wrong reading the file");

        println!("{}", config);

        let config: Config = serde_json::from_str(config).expect("JSON was not well-formatted");

        return config;
    };
}
