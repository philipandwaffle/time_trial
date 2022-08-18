use std::ops::{Div, Sub};

use bevy::{prelude::*, math::vec2, utils::tracing::enabled};

use crate::{player::PlayerController, movement::Movable};

#[derive(Component)]
pub struct CollisionBox{    
    pub size: Vec2,
    pub colliding_with: Vec<Entity>,
    pub enabled: bool,
}
impl CollisionBox{
    pub fn set_enabled(&mut self, enabled: bool){
        self.enabled = enabled;
    }
}
impl Default for CollisionBox{
    fn default() -> Self {
        Self { 
            size: Default::default(), 
            colliding_with: vec![], 
            enabled: true
        }
    }
}

#[derive(Component)]
pub struct Pushable{
    pub push_vel: f32,
}

pub fn detect_collisions(
    mut collidables: Query<(Entity, &Transform, &mut CollisionBox)>
){
    // vector containing (entity, entity centre pos, entity size, colliding entities vec) tupples
    let mut entities: Vec<(Entity, Vec2, Vec2)> = Vec::new();
    for (e, t, cb) in collidables.iter(){
        if !cb.enabled{
            continue;
        }
        entities.push((e, vec2(t.translation.x, t.translation.y), cb.size));
    }

    for i in 0..entities.len(){        
        let a_pos: Vec2 = entities[i].1 + Vec2::div(entities[i].2 , 2.);
        let a_neg: Vec2 = entities[i].1 - Vec2::div(entities[i].2 , 2.);
        
        let mut colliding_with: Vec<Entity> = vec![];
        for j in 0..entities.len(){
            if i != j {
                let b_pos: Vec2 = entities[j].1 + Vec2::div(entities[j].2 , 2.);
                let b_neg: Vec2 = entities[j].1 - Vec2::div(entities[j].2 , 2.);
                
                // is a's top right vertex right of b's bottom left vertex
                // is a's top right vertex above of b's bottob left vertex
                // is a's bottom left vertex left of b's top right vertex
                // is a's bottom left vertex below of b's top right vertex
                
                if a_pos.x > b_neg.x && a_pos.y > b_neg.y && a_neg.x < b_pos.x && a_neg.y < b_pos.y {
                    
                    colliding_with.push(entities[j].0);
                    println!("Collision detected");
                    let cv = entities[j].1 - entities[i].1;
                }
            }            
        }
        if let Ok((e, t, mut cb)) = collidables.get_mut(entities[i].0){
            cb.colliding_with = colliding_with;
        }
    }
}

pub fn push_pushables(
    mut pushables: Query<(&Transform, &CollisionBox, &mut Movable, &Pushable)>,
    collidables: Query<(&Transform, &CollisionBox)>
){    
    for (t, cb, mut m, p) in pushables.iter_mut(){
        let pos = t.translation;
        let collided_with = &cb.colliding_with;

        let mut push_vec = vec2(0., 0.);
        for e in collided_with.iter(){            
            push_vec += -(collidables.get(*e).unwrap().0.translation - pos).truncate().normalize();
        }
        m.apply_temp_vel(push_vec * p.push_vel);
    }
}

