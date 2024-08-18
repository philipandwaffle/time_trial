use bevy::{
    math::vec3,
    prelude::{
        default, Bundle, Camera2dBundle, Commands, Component, Entity, OrthographicProjection,
        Transform,
    },
};

use crate::consts::CAMERA_Z_OFFSET;

#[derive(Component)]
pub struct PlayerCam;

#[derive(Bundle)]
pub struct PlayerCamBundle {
    camera_2d_bundle: Camera2dBundle,
    player_cam: PlayerCam,
}
impl PlayerCamBundle {
    pub fn new(scale: f32) -> Self {
        return Self {
            camera_2d_bundle: Camera2dBundle {
                transform: Transform::from_translation(vec3(0.0, 0.0, CAMERA_Z_OFFSET)),
                projection: OrthographicProjection {
                    scale: scale,
                    ..default()
                },
                ..default()
            },
            player_cam: PlayerCam,
        };
    }

    pub fn spawn(self, commands: &mut Commands) -> Entity {
        return commands.spawn(self).id();
    }
}
