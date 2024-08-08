use bevy::prelude::{
    default, Bundle, Camera2dBundle, Commands, Component, Entity, OrthographicProjection,
};

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
