use crate::player::player_bundle::PlayerPlugin;
use bevy::app::{App, Startup};
use bevy::prelude::{default, Commands, ImagePlugin, PluginGroup};
use bevy::transform::commands;
use bevy::window::{Window, WindowMode, WindowPlugin, WindowPosition};
use bevy::{log::LogPlugin, DefaultPlugins};
use bevy_rapier2d::prelude::*;
use configuration::display::DisplayConfig;
use configuration::{Config, ConfigPlugin};
use consts::DISPLAY_CFG_PATH;
use handles::HandlesPlugin;
use level::manager::LevelManagerPlugin;
use ui::selection_list::{LevelPackItem, UIListBundle};

mod configuration;
mod consts;
mod handles;
mod level;
mod player;
mod ui;

fn main() {
    let display_config = DisplayConfig::load_cfg(DISPLAY_CFG_PATH);
    let window = Window {
        title: "time_trial".into(),
        resolution: (display_config.resolution.x, display_config.resolution.y).into(),
        mode: WindowMode::Windowed,
        position: WindowPosition::At(display_config.pos),
        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
        // prevent_default_event_handling: false,
        ..default()
    };

    App::new()
        .insert_resource(RapierConfiguration::new(0.0))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(window),
                    ..default()
                })
                // don't use linear sampling as image textures will be blurry
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    // Uncomment this to override the default log settings:
                    level: bevy::log::Level::WARN,
                    ..default()
                }),
        )
        .add_plugins((
            ConfigPlugin,
            HandlesPlugin,
            LevelManagerPlugin,
            PlayerPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        ))
        .add_systems(Startup, testing)
        .run();
}

fn testing(mut commands: Commands) {
    UIListBundle::new().spawn(
        &mut commands,
        vec![Box::new(LevelPackItem::new("name", 0.5, 0.5, "foo"))],
    );
}
