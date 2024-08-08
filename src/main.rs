use std::f32::consts::PI;

use crate::player::player_bundle::PlayerPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_rapier2d::prelude::*;
use configuration::ConfigPlugin;

// use crate::level::LevelControllerPlugin;

mod configuration;
mod consts;
mod level;
mod player;

fn main() {
    App::new()
        .insert_resource(RapierConfiguration::new(0.0))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    // primary_window: None,
                    // exit_condition: bevy::window::ExitCondition::DontExit,
                    primary_window: Some(Window {
                        title: "primordial_pixels".into(),
                        resolution: (2560.0 * 0.5, 1440.0).into(),
                        mode: WindowMode::Windowed,
                        position: WindowPosition::At(IVec2::new(0, 0)),
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        ..default()
                    }),
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
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
            PlayerPlugin,
            // LevelControllerPlugin,
        ))
        .add_systems(Startup, spawn_scene)
        .run();
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
