use bevy::{
    math::vec2,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;

use crate::{
    configuration::{key_bindings::KeyBinds, player::PlayerConfig},
    consts::PLAYER_Z_OFFSET,
    handles::Handles,
};

use super::player_camera::PlayerCamBundle;

#[derive(Component, Clone)]
pub struct Player;

#[derive(Bundle, Clone)]
pub struct PlayerBundle {
    pub material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub damping: Damping,
    pub locked_axis: LockedAxes,
    pub velocity: Velocity,
    pub player: Player,
}
impl PlayerBundle {
    pub fn new(
        material: &Handle<ColorMaterial>,
        mesh: &Mesh2dHandle,
        collider_radius: f32,
        linear_damping: f32,
        angular_damping: f32,
        pos: Vec2,
    ) -> Self {
        return Self {
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_translation(pos.extend(PLAYER_Z_OFFSET)),
                ..default()
            },
            collider: Collider::ball(collider_radius),
            rigid_body: RigidBody::Dynamic,
            damping: Damping {
                linear_damping,
                angular_damping,
            },
            locked_axis: LockedAxes::ROTATION_LOCKED,
            velocity: Velocity::default(),
            player: Player,
        };
    }
    pub fn spawn(self, commands: &mut Commands) -> Entity {
        return commands.spawn(self).id();
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}
fn spawn_player(mut commands: Commands, player_config: Res<PlayerConfig>, handles: Res<Handles>) {
    let cam_ent = PlayerCamBundle::new(player_config.cam_scale).spawn(&mut commands);
    let player_ent = PlayerBundle::new(
        &handles.player_material,
        &handles.player_mesh,
        player_config.collider_radius,
        player_config.linear_damping,
        player_config.angular_damping,
        vec2(0.0, 0.0),
    )
    .spawn(&mut commands);
    commands.get_entity(player_ent).unwrap().add_child(cam_ent);
}

fn move_player(
    mut players: Query<&mut Velocity, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    key_binds: Res<KeyBinds>,
    player_config: Res<PlayerConfig>,
) {
    if let Ok(mut player) = players.get_single_mut() {
        let mut vel = player.linvel;
        let move_speed = player_config.move_speed;

        if keys.pressed(key_binds.up.0) {
            vel.y += move_speed;
        }
        if keys.pressed(key_binds.left.0) {
            vel.x -= move_speed;
        }
        if keys.pressed(key_binds.down.0) {
            vel.y -= move_speed;
        }
        if keys.pressed(key_binds.right.0) {
            vel.x += move_speed;
        }

        player.linvel = vel;
    }
}
