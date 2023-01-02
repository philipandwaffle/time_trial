use std::f32::consts::PI;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_rapier2d::prelude::*;
use player::PlayerPlugin;

use crate::config::CONFIGURATION;
use crate::input::InputPlugin;
use crate::level::LevelControllerPlugin;

mod config;
mod input;
mod level;
mod player;

#[macro_use]
extern crate lazy_static;

fn main() {
    let log_level = match CONFIGURATION.logging.level.as_str() {
        "error" => bevy::log::Level::ERROR,
        "warn" => bevy::log::Level::WARN,
        "debug" => bevy::log::Level::DEBUG,
        "info" => bevy::log::Level::INFO,
        "trace" => bevy::log::Level::TRACE,
        _ => panic!(
            "{} is not a valid log level",
            CONFIGURATION.logging.level.as_str()
        ),
    };
    // error!("I'm an ERROR");
    // warn!("I'm an WARN");
    // debug!("I'm an DEBUG");
    // info!("I'm an INFO");
    // trace!("I'm an TRACE");

    App::new()
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 640.,
                        height: 360.,
                        title: "Testing".to_string(),
                        resizable: false,
                        ..default()
                    },
                    ..default()
                })
                .set(LogPlugin {
                    level: log_level,
                    filter: CONFIGURATION.logging.filter.clone(),
                }),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(InspectableRapierPlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(LevelControllerPlugin)
        .add_startup_system(spawn_camera)
        //.add_startup_system(spawn_scene)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Name::new("Camera")).insert(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 4.0,
            ..default()
        },
        ..default()
    });
}

fn spawn_scene(mut commands: Commands) {
    commands
        .spawn(Name::new("ground_right"))
        .insert(TransformBundle::from_transform(Transform {
            translation: Vec3::new(250.0, 0.0, 0.0),
            rotation: Quat::from_rotation_z(PI / 4.0),
            ..default()
        }))
        .insert(Collider::cuboid(500.0, 50.0));
    commands
        .spawn(Name::new("ground_left"))
        .insert(TransformBundle::from_transform(Transform {
            translation: Vec3::new(-250.0, 0.0, 0.0),
            rotation: Quat::from_rotation_z(3.0 * PI / 4.0),
            ..default()
        }))
        .insert(Collider::cuboid(500.0, 50.0));
}

fn spawn_ball(commands: &mut Commands, x: f32, y: f32) {
    commands
        .spawn(Name::new("Ball"))
        .insert(PhysicsObjectBundle {
            transform_bundle: TransformBundle::from(Transform::from_xyz(x, y, 0.0)),
            collider: Collider::ball(20.0),
            ..default()
        });
}

#[derive(Bundle, Clone, Debug)]
pub struct PhysicsObjectBundle {
    pub transform_bundle: TransformBundle,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub damping: Damping,
    pub restitution: Restitution,
}
impl Default for PhysicsObjectBundle {
    fn default() -> Self {
        Self {
            transform_bundle: TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
            collider: Collider::ball(50.0),
            rigid_body: RigidBody::Dynamic,
            damping: Damping {
                linear_damping: 1.0,
                angular_damping: 1.0,
            },
            restitution: Restitution {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Average,
            },
        }
    }
}
