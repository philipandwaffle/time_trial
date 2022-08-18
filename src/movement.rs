use bevy::{prelude::*, core_pipeline::draw_3d_graph, ecs::query};


// Contains the relevent information to move an object
#[derive(Component)]
pub struct Movable{
    pub acc: Vec2,
    pub vel: Vec2,
    pub temp_vel: Option<Vec2>,
}
impl Movable{
    pub fn modify_vel(&mut self){
        self.vel += self.acc;

        if self.temp_vel.is_some(){
            self.vel += self.temp_vel.unwrap();            
        }
    }

    pub fn apply_temp_vel(&mut self, temp_vel: Vec2){
        self.temp_vel = Some(temp_vel);
    }
    pub fn remove_temp_vel(&mut self){
        if self.temp_vel.is_some(){
            self.vel -= self.temp_vel.unwrap();
            self.temp_vel = None;
        }
    }
}
impl Default for Movable{
    fn default() -> Self {
        Self { 
            acc: Default::default(), 
            vel: Default::default(), 
            temp_vel: Default::default() 
        }
    }
}

// Applies drag to movable objects by multipling the current velocity by the drag
#[derive(Component)]
pub struct Drag{
    pub drag: f32,
    pub enabled: bool,
}
impl Drag{
    pub fn set_drag(&mut self, drag: f32) -> bool{
        if drag < 1. && drag > 0.{
            self.drag = drag;
            return true;
        }            
        return false;        
    }
    pub fn set_enabled(&mut self, enabled: bool){
        self.enabled = enabled;
    }    
}

// Supply a function that takes a vec2 and returns a vec2 to create your own constraint
#[derive(Component)]
pub struct VelConstraint{
    pub constrain_fn: fn(Vec2) -> Vec2,
}

// Handles the movement of movable objects taking into account temp velocities and constraint functions
pub fn move_movables(
    mut movables: Query<(&mut Transform, &mut Movable, Option<&VelConstraint>)>
){
    for (mut t, mut m, vc) in movables.iter_mut(){
        m.modify_vel();
        // constrain the velocity if it needs to be constrained
        if vc.is_some(){
            t.translation += (vc.unwrap().constrain_fn)(m.vel).extend(0.);
        }else{
            t.translation += m.vel.extend(0.);   
        }        
        m.remove_temp_vel();
    }
}

pub fn apply_drag(
    mut drag_affected: Query<(&mut Movable, &Drag)>
){
    for (mut m, d) in drag_affected.iter_mut(){
        if d.enabled{
            m.vel *= d.drag;
        }
    }
}