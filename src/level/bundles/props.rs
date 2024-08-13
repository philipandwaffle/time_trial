use bevy::{
    asset::Handle,
    math::Vec2,
    prelude::Bundle,
    sprite::{ColorMaterial, Mesh2dHandle},
};
use bevy_rapier2d::prelude::{Collider, Damping, LockedAxes, RigidBody, Velocity};

#[derive(Bundle)]
pub struct PhysicsPropBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub damping: Damping,
    pub locked_axis: LockedAxes,
    pub velocity: Velocity,
}
impl PhysicsPropBundle {
    pub fn new(
        material: &Handle<ColorMaterial>,
        mesh: &Mesh2dHandle,
        collider: Collider,
        pos: Vec2,
        z_rot: f32,
        linear_damping: f32,
        angular_damping: f32,
    ) -> Self {
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
    pub fn spawn() -> Self {}
}
