use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{config::CONFIGURATION, input::InputStates};

#[derive(Component, Clone, Debug)]
pub struct Player;

#[derive(Bundle, Clone, Debug)]
pub struct PlayerBundle {
    pub transform_bundle: TransformBundle,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub damping: Damping,
    pub restitution: Restitution,
    pub velocity: Velocity,
    pub player: Player,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            transform_bundle: TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
            collider: Collider::ball(50.0),
            rigid_body: RigidBody::Dynamic,
            damping: Damping {
                linear_damping: CONFIGURATION.player.linear_damping,
                angular_damping: CONFIGURATION.player.angular_damping,
            },
            restitution: Restitution {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Average,
            },
            velocity: Velocity::default(),
            player: Player,
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_player).add_system(move_player);
    }
}
fn spawn_player(mut commands: Commands) {
    commands
        .spawn(Name::new("Player"))
        .insert(PlayerBundle::default());
}

//static MOVE_SPEED: f32 = 100.0;
fn move_player(mut players: Query<&mut Velocity, With<Player>>, input: Res<InputStates>) {
    for mut player in players.iter_mut() {
        let mut vel = player.linvel;

        if input.move_up {
            vel.y += &CONFIGURATION.player.move_speed;
        }
        if input.move_left {
            vel.x -= &CONFIGURATION.player.move_speed
        }
        if input.move_down {
            vel.y -= &CONFIGURATION.player.move_speed;
        }
        if input.move_right {
            vel.x += &CONFIGURATION.player.move_speed;
        }

        // if input.pressed(KeyCode::E) {
        //     spawn_ball(&mut commands, 0.0, 10.0);
        // }

        player.linvel = vel;
    }
}
