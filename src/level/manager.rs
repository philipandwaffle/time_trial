use std::f32::consts::PI;

use bevy::{
    app::{Plugin, Startup},
    math::vec2,
    prelude::{Commands, Entity, Res, ResMut, Resource},
};

use crate::{
    configuration::{level::LevelConfig, Config},
    handles::Handles,
};

use super::{blue_print::*, gate::*, input::ButtonType, logic_tree::*};

pub struct LevelManagerPlugin;
impl LevelManagerPlugin {
    pub fn gen_blueprint() -> Blueprint {
        let walls = vec![
            WallBluePrint::new(vec2(500.0, -50.0), 0.0, vec2(500.0, 5.0)),
            WallBluePrint::new(vec2(500.0, 50.0), 0.0, vec2(500.0, 5.0)),
            WallBluePrint::new(vec2(50.0, 0.0), 0.0, vec2(5.0, 50.0)),
            WallBluePrint::new(vec2(-50.0, 0.0), 0.0, vec2(5.0, 50.0)),
        ];

        let inputs = vec![InputBluePrint::Button(ButtonBluePrint::new(
            vec2(100.0, 0.0),
            5.0,
            ButtonType::PressButton,
        ))];

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

        let bp = Blueprint {
            walls,
            inputs,
            outputs: vec![],
            logic_tree,
        };

        return bp;
    }
}
impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // Self::gen_blueprint().save_cfg("levels/001.json");

        app.insert_resource(LevelManager {
            cur_level_root: None,
        })
        .add_systems(Startup, load_level);
    }
}

pub fn load_level(
    mut commands: Commands,
    handles: Res<Handles>,
    level_config: Res<LevelConfig>,
    mut level_manager: ResMut<LevelManager>,
) {
    if level_config.gen_on_start {
        let blueprint = LevelManagerPlugin::gen_blueprint();
        if level_config.save_on_start {
            blueprint.save_cfg(&format!("{}/{}", level_config.dir, level_config.cur_file));
        }

        level_manager.cur_level_root = Some(blueprint.spawn(&mut commands, &handles));
    } else {
        level_manager.cur_level_root = Some(
            Blueprint::load_cfg(&format!("{}/{}", level_config.dir, level_config.cur_file))
                .spawn(&mut commands, &handles),
        );
    }
}

#[derive(Resource)]
pub struct LevelManager {
    cur_level_root: Option<Entity>,
}
