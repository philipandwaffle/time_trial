use bevy::{
    asset::Handle,
    math::{vec3, Quat, Vec2, Vec3},
    prelude::{
        default, Bundle, Commands, Entity, InheritedVisibility, Transform, TransformBundle,
        VisibilityBundle,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::{Collider, Sensor};

use super::input::{PressButton, ToggleButton};

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

#[derive(Bundle)]
pub struct ToggleButtonBundle {
    button_bundle: ButtonBundle,
    toggle_button: ToggleButton,
}
impl ToggleButtonBundle {
    pub fn new(
        material: &Handle<ColorMaterial>,
        mesh: &Mesh2dHandle,
        radius: f32,
        pos: Vec2,
    ) -> Self {
        return Self {
            button_bundle: ButtonBundle::new(material, mesh, radius, pos),
            toggle_button: ToggleButton::default(),
        };
    }

    pub fn spawn(self, commands: &mut Commands) -> Entity {
        return commands.spawn(self).id();
    }
}

#[derive(Bundle)]
pub struct PressButtonBundle {
    button_bundle: ButtonBundle,
    press_button: PressButton,
}
impl PressButtonBundle {
    pub fn new(
        material: &Handle<ColorMaterial>,
        mesh: &Mesh2dHandle,
        radius: f32,
        pos: Vec2,
    ) -> Self {
        return Self {
            button_bundle: ButtonBundle::new(material, mesh, radius, pos),
            press_button: PressButton::default(),
        };
    }

    pub fn spawn(self, commands: &mut Commands) -> Entity {
        return commands.spawn(self).id();
    }
}

#[derive(Bundle)]
pub struct ButtonBundle {
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    collider: Collider,
    sensor: Sensor,
}
impl ButtonBundle {
    pub fn new(
        material: &Handle<ColorMaterial>,
        mesh: &Mesh2dHandle,
        radius: f32,
        pos: Vec2,
    ) -> Self {
        return Self {
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform {
                    translation: pos.extend(0.0),
                    scale: vec3(radius, radius, 1.0),
                    ..default()
                },
                ..default()
            },
            collider: Collider::ball(0.5),
            sensor: Sensor,
        };
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
