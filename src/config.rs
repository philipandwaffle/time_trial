use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub logging: Logging,
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
