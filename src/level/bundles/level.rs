use bevy::{
    math::vec3,
    prelude::{default, Bundle, Commands, Entity, Transform, TransformBundle, VisibilityBundle},
};

use super::consts::LEVEL_Z_OFFSET;

#[derive(Bundle, Default)]
pub struct LevelRootBundle {
    transform_bundle: TransformBundle,
    visibility_bundle: VisibilityBundle,
}
impl LevelRootBundle {
    pub fn new() -> Self {
        return Self {
            transform_bundle: TransformBundle::from_transform(Transform {
                translation: vec3(0.0, 0.0, LEVEL_Z_OFFSET),
                ..default()
            }),
            visibility_bundle: VisibilityBundle::default(),
        };
    }
    pub fn spawn(self, commands: &mut Commands) -> Entity {
        return commands.spawn(self).id();
    }
}
