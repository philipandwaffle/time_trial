use bevy::{DefaultPlugins, core::Time, window::{Windows, WindowDescriptor}, ecs::bundle, render::texture::DEFAULT_IMAGE_HANDLE, math::{vec2, vec3}};
use bevy::prelude::*;
use player::PlayerController;
use crate::collision::*;
use crate::player::*;
use crate::movement::*;
use crate::resources::*;
use crate::interactables::*;

mod player;
mod collision;
mod movement;
mod resources;
mod interactables;

pub struct WinSize {
	pub w: f32,
	pub h: f32,
}

fn main() {
    println!("Hello, world!");

    App::new()
        .insert_resource(WindowDescriptor {
			title: "Physics Engine".to_string(),
            mode: bevy::window::WindowMode::BorderlessFullscreen,
			..Default::default()
		})
        .insert_resource(Bindings::default())
        .insert_resource(TimeStep{
            time: Default::default(),
            time_step: 1./128.
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_startup_system(spawn_entities)
        .add_system(move_player)
        .add_system(move_movables)
        .add_system(apply_drag)
        .add_system(detect_collisions)        
        .add_system(push_pushables)                
        .run();
}

fn setup_system(
	mut commands: Commands,
	mut windows: ResMut<Windows>,
) {
	// camera
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	// capture window size
	let window = windows.get_primary_mut().unwrap();
	let (win_w, win_h) = (window.width(), window.height());

	// position window (for tutorial)
	// window.set_position(IVec2::new(2780, 4900));

	// add WinSize resource
	let win_size = WinSize { w: win_w, h: win_h };
	commands.insert_resource(win_size);
}

fn spawn_entities(
    mut commands: Commands,
){  
    // player
    commands.spawn_bundle((       
        Transform {
            translation: vec3(0., 0., 0.),
            scale: vec3(100., 100., 100.),
            rotation: Quat::IDENTITY,
        },
        Sprite {
            color: Color::rgb(1., 1., 1.),
            ..Default::default()
        },                                                                                                                                                                                                                                                                                                                                                      
        GlobalTransform::default(),
        Visibility::default(),
        DEFAULT_IMAGE_HANDLE.typed::<Image>(),
        PlayerController {
            speed: 2.,
            max_speed: 4.,
        },
        Movable::default(),
        Pushable{
            push_vel: 2.,
        },
        Drag{
            drag: 0.9,
            enabled: true,
        },
        CollisionBox{
            size: vec2(100., 100.),
            ..Default::default()
        }
    ));

    // box
    commands.spawn_bundle((       
        Transform {
            translation: vec3(200., 200., 0.),
            scale: vec3(100., 100., 100.),
            rotation: Quat::IDENTITY,
        },
        Sprite {
            color: Color::rgb(1., 0., 0.),
            ..Default::default()
        },
        GlobalTransform::default(),
        Visibility::default(),
        DEFAULT_IMAGE_HANDLE.typed::<Image>(),
        CollisionBox{
            size: vec2(100., 100.),
            ..Default::default()
        },
        Movable::default(),
        Pushable{
            push_vel: 5.,
        },
        VelConstraint{
            constrain_fn: |vel| -> Vec2{
                // move along the x-axis
                if vel.x.abs() > vel.y.abs(){
                    return vec2(vel.x, 0.);                    
                }
                // move along the y-axis
                else{
                    return vec2(0., vel.y);
                }                
            },
        }
    ));

    // Box 2
    commands.spawn_bundle((       
        Transform {
            translation: vec3(-200., 200., 0.),
            scale: vec3(100., 100., 100.),
            rotation: Quat::IDENTITY,
        },
        Sprite {
            color: Color::rgb(1., 0., 0.),
            ..Default::default()
        },
        GlobalTransform::default(),
        Visibility::default(),
        DEFAULT_IMAGE_HANDLE.typed::<Image>(),
        CollisionBox{
            size: vec2(100., 100.),
            ..Default::default()
        },
        Movable::default(),
        Pushable{
            push_vel: 5.,
        },
        VelConstraint{
            constrain_fn: |vel| -> Vec2{
                // move along the x-axis
                if vel.x.abs() > vel.y.abs(){                    
                    return vec2(vel.x, 0.);                    
                }
                // move along the y-axis
                else{
                    return vec2(0., vel.y);
                }                
            },
        }
    ));
}
