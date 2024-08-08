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
    });
}