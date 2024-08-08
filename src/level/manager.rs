use std::f32::consts::PI;

use bevy::{
    app::{Plugin, Startup},
    math::vec2,
    prelude::{Commands, Entity, Res, ResMut, Resource},
};

use crate::{configuration::Config, handles::Handles};

use super::{blue_print::*, gate::*, logic_tree::*};

pub struct LevelManagerPlugin;
impl LevelManagerPlugin {
    pub fn gen_blueprint() -> BluePrint {
        let logic_tree = LogicTree::new(
            vec![
                vec![
                    GateTypes::OrGate(OrGate::default()),
                    GateTypes::OrGate(OrGate::default()),
                ],
                vec![GateTypes::AndGate(AndGate::default())],
            ],
            vec![vec![0, 1, 2, 3], vec![0, 1], vec![0]],
        );

        let bp = BluePrint {
            logic_tree: logic_tree,
            inputs: vec![],
            outputs: vec![],
            walls: vec![
                WallBluePrint::new(vec2(250.0, 0.0), PI / 4.0, vec2(500.0, 50.0)),
                WallBluePrint::new(vec2(-250.0, 0.0), 3.0 * PI / 4.0, vec2(500.0, 50.0)),
            ],
        };

        return bp;
    }
}
impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(LevelManager {
            cur_level_root: None,
        })
        .add_systems(Startup, load_level);
    }
}

pub fn load_level(
    mut commands: Commands,
    handles: Res<Handles>,
    mut level_manager: ResMut<LevelManager>,
) {
    level_manager.cur_level_root =
        Some(BluePrint::load_cfg("levels/000.json").spawn(&mut commands, &handles));
    // level_manager.cur_level_root =
    //     Some(LevelManagerPlugin::gen_blueprint().spawn(&mut commands, &handles));
}

#[derive(Resource)]
pub struct LevelManager {
    cur_level_root: Option<Entity>,
    
}
