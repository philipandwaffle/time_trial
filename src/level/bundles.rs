use bevy::{
    asset::Handle,
    math::{vec3, Quat, Vec2, Vec3},
    prelude::{
        default, Bundle, Commands, Entity, InheritedVisibility, Transform, TransformBundle,
        VisibilityBundle,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::Collider;

#[derive(Bundle)]
pub struct WallBundle {
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    collider: Collider,
}
impl WallBundle {
    pub fn new(
        material: &Handle<ColorMaterial>,
        mesh: &Mesh2dHandle,
        pos: Vec2,
        z_rot: f32,
        shape: Vec2,
    ) -> Self {
        return Self {
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform {
                    translation: pos.extend(-1.0),
                    rotation: Quat::from_rotation_z(z_rot),
                    scale: shape.extend(1.0),
                },
                ..default()
            },
            collider: Collider::cuboid(0.5, 0.5),
        };
    }

    pub fn spawn(self, commands: &mut Commands) -> Entity {
        return commands.spawn(self).id();
    }
}

#[derive(Bundle, Default)]
pub struct LevelRootBundle {
    transform_bundle: TransformBundle,
    visibility_bundle: VisibilityBundle,
}
impl LevelRootBundle {
    pub fn new() -> Self {
        return Self {
            transform_bundle: TransformBundle::from_transform(Transform {
                translation: vec3(0.0, 0.0, -1.0),
                ..default()
            }),
            visibility_bundle: VisibilityBundle::default(),
        };
    }
    pub fn spawn(self, commands: &mut Commands) -> Entity {
        return commands.spawn(self).id();
    }
}
