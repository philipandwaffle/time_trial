use bevy::{prelude::*, ui::entity};

#[derive(Component)]
pub struct Input{
    pub activation_fn: fn () -> bool,
}

// A logic gate will hold a refrence to each input
#[derive(Component)]
pub struct Gate{
    pub inputs: Vec<Entity>,
    pub logic_fn: fn (inputs: Vec<bool>) -> bool,   
}
impl Gate{
    pub fn get_input_states(&self) -> Vec<bool>{
        let inputs: Query<&Input>;
        return self.inputs.iter().map(|entity| {
            (inputs.get(*entity).unwrap().activation_fn)()
        }).collect::<Vec<bool>>();
    }
}

#[derive(Component)]
pub struct Output{
    pub input_entity_id: u32,
    pub has_gate: bool,
    pub toggle_state: fn(),
}
impl Output{
    pub fn add_input(&mut self, ){

    }
    pub fn check_state(&self){
        // the gate's state needs to be calculated
        if self.has_gate{
            
        }else{

        }
    }
}