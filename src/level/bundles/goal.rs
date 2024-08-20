use bevy::{
    asset::Handle,
    math::{Quat, Vec2},
    prelude::{default, Bundle, Commands, Entity, Transform},
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::{Collider, Sensor};

use crate::consts::GOAL_Z_OFFSET;

#[derive(Bundle)]
pub struct GoalBundle {
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    collider: Collider,
    sensor: Sensor,
}
impl GoalBundle {
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
                    translation: pos.extend(GOAL_Z_OFFSET),
                    rotation: Quat::from_rotation_z(z_rot),
                    scale: shape.extend(1.0),
                },
                ..default()
            },
            collider: Collider::cuboid(0.5, 0.5),
            sensor: Sensor,
        };
    }
    pub fn spawn(self, commands: &mut Commands) -> Entity {
        return commands.spawn(self).id();
    }
}
