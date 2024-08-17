use bevy::{
    asset::Handle,
    math::{Quat, Vec2},
    prelude::{default, Bundle, Commands, Entity, Transform},
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::{Collider, Damping, LockedAxes, RigidBody, Velocity};

use crate::{consts::PROP_Z_OFFSET, level::time_shift::TimeShift};

#[derive(Bundle)]
pub struct BoxBundle {
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    physics_prop_bundle: PhysicsPropBundle,
    time_shift: TimeShift,
}
impl BoxBundle {
    pub fn new(
        material: &Handle<ColorMaterial>,
        mesh: &Mesh2dHandle,
        shape: Vec2,
        pos: Vec2,
        z_rot: f32,
        linear_damping: f32,
        angular_damping: f32,
    ) -> Self {
        return Self {
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform {
                    translation: pos.extend(PROP_Z_OFFSET),
                    rotation: Quat::from_rotation_z(z_rot),
                    scale: shape.extend(1.0),
                },
                ..default()
            },
            physics_prop_bundle: PhysicsPropBundle::new(
                Collider::cuboid(0.5, 0.5),
                linear_damping,
                angular_damping,
            ),
            time_shift: TimeShift::new(pos),
        };
    }
    pub fn spawn(self, commands: &mut Commands) -> Entity {
        return commands.spawn(self).id();
    }
}

#[derive(Bundle)]
pub struct PhysicsPropBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub damping: Damping,
    pub locked_axis: LockedAxes,
    pub velocity: Velocity,
}
impl PhysicsPropBundle {
    pub fn new(collider: Collider, linear_damping: f32, angular_damping: f32) -> Self {
        return Self {
            collider,
            rigid_body: RigidBody::Dynamic,
            damping: Damping {
                linear_damping,
                angular_damping,
            },
            locked_axis: LockedAxes::ROTATION_LOCKED,
            velocity: Velocity::default(),
        };
    }
}
