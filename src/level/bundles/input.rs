use bevy::{
    asset::Handle,
    math::{vec3, Vec2},
    prelude::{default, Bundle, Commands, Entity, Transform},
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::{Collider, Sensor};

use crate::{
    consts::INPUT_Z_OFFSET,
    level::input::{PressButton, ToggleButton},
};

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
        on_key: &str,
        off_key: &str,
    ) -> Self {
        return Self {
            button_bundle: ButtonBundle::new(material, mesh, radius, pos),
            toggle_button: ToggleButton::new(on_key, off_key),
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
        on_key: &str,
        off_key: &str,
    ) -> Self {
        return Self {
            button_bundle: ButtonBundle::new(material, mesh, radius, pos),
            press_button: PressButton::new(on_key, off_key),
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
                    translation: pos.extend(INPUT_Z_OFFSET),
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
