use bevy::{prelude::*, math::vec2};
use std::{collections::HashMap, ops::Mul};

use crate::movement::{Movable, Drag};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Action{
    up,
    down,
    left, 
    right,
    switch_time    
}

pub struct Bindings{
    pub binds: HashMap<KeyCode, Action>
}
impl Default for Bindings {
    fn default() -> Self {
        return Self { 
            binds: HashMap::from([
                (KeyCode::W, Action::up),
                (KeyCode::S, Action::down),
                (KeyCode::A, Action::left),
                (KeyCode::D, Action::right),
                (KeyCode::E, Action::switch_time),
            ])
        }
    }
}

#[derive(Component)]
pub struct PlayerController{
    pub speed: f32,
    pub max_speed: f32,
}


pub fn move_player(
    keys: Res<Input<KeyCode>>,
    bindings: Res<Bindings>,
    mut players: Query<(&PlayerController, &mut Movable)>,
){    
    for (pc, mut m) in players.iter_mut(){
        let mut delta_x: i32 = 0;
        let mut delta_y: i32 = 0;
        for (k, v) in bindings.binds.iter(){
            if keys.pressed(*k){
                match v{
                    Action::up => delta_y += 1,
                    Action::down => delta_y -= 1,
                    Action::left => delta_x -= 1,
                    Action::right => delta_x += 1,
                    Action::switch_time => todo!(),
                }                
            }
        }
        let move_dir = vec2(delta_x as f32, delta_y as f32).normalize_or_zero();        
        let vel = ((move_dir.mul(pc.speed)) + m.vel).clamp(vec2(-pc.max_speed, -pc.max_speed), vec2(pc.max_speed, pc.max_speed));
        m.vel = vel;
    }
}