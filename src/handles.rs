use bevy::{
    app::{Plugin, PreStartup},
    ecs::system::Res,
    prelude::{Assets, Circle, Commands, Handle, Mesh, Rectangle, ResMut, Resource},
    sprite::{ColorMaterial, Mesh2dHandle},
};

use crate::configuration::{material::MaterialConfig, mesh::MeshConfig};

#[derive(Resource)]
pub struct Handles {
    pub wall_mesh: Mesh2dHandle,
    pub wall_material: Handle<ColorMaterial>,
    pub player_mesh: Mesh2dHandle,
    pub player_material: Handle<ColorMaterial>,
    pub button_mesh: Mesh2dHandle,
    pub press_button_material: Handle<ColorMaterial>,
    pub toggle_button_material: Handle<ColorMaterial>,
    pub door_mesh: Mesh2dHandle,
    pub door_material: Handle<ColorMaterial>,
    pub square_mesh: Mesh2dHandle,
    pub circle_mesh: Mesh2dHandle,
}

pub struct HandlesPlugin;
impl Plugin for HandlesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreStartup, setup_handles);
    }
}

pub fn setup_handles(
    mut commands: Commands,
    mesh_config: Res<MeshConfig>,
    materials_config: Res<MaterialConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Handles {
        wall_mesh: meshes
            .add(Rectangle::new(mesh_config.wall[0], mesh_config.wall[1]))
            .into(),
        wall_material: materials.add(materials_config.wall.to_color_mat()),
        player_mesh: meshes.add(Circle::new(mesh_config.player)).into(),
        player_material: materials.add(materials_config.player.to_color_mat()),
        button_mesh: meshes.add(Circle::new(mesh_config.button)).into(),
        press_button_material: materials.add(materials_config.press_button.to_color_mat()),
        toggle_button_material: materials.add(materials_config.toggle_button.to_color_mat()),
        door_mesh: meshes
            .add(Rectangle::new(mesh_config.door[0], mesh_config.door[1]))
            .into(),
        door_material: materials.add(materials_config.door.to_color_mat()),
        square_mesh: meshes.add(Rectangle::new(1.0, 1.0)).into(),
        circle_mesh: meshes.add(Circle::new(0.5)).into(),
    });
}
