use crate::player::player_bundle::PlayerPlugin;
use bevy::app::App;
use bevy::math::IVec2;
use bevy::prelude::{default, ImagePlugin, PluginGroup};
use bevy::window::{Window, WindowMode, WindowPlugin, WindowPosition};
use bevy::{log::LogPlugin, DefaultPlugins};
use bevy_rapier2d::prelude::*;
use configuration::ConfigPlugin;
use handles::HandlesPlugin;
use level::manager::LevelManagerPlugin;

mod configuration;
mod consts;
mod handles;
mod level;
mod player;

fn main() {
    let window = Window {
        title: "time_trial".into(),
        resolution: (2560.0 * 0.5, 1440.0).into(),
        mode: WindowMode::Windowed,
        position: WindowPosition::At(IVec2::new(0, 0)),
        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
        prevent_default_event_handling: false,
        ..default()
    };
    App::new()
        .insert_resource(RapierConfiguration::new(0.0))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    // primary_window: None,
                    // exit_condition: bevy::window::ExitCondition::DontExit,
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
            RapierDebugRenderPlugin::default(),
        ))
        .run();
}
