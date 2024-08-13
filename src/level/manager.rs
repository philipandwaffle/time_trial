use bevy::{
    app::{Plugin, Startup, Update},
    math::vec2,
    prelude::{Commands, Query, Res, ResMut, Resource},
};
use bevy_trait_query::One;

use crate::{
    configuration::{level::LevelConfig, Config},
    handles::Handles,
};

use super::{
    blueprints::{
        input::{ButtonBlueprint, InputBlueprint},
        level::Blueprint,
        output::{DoorBlueprint, OutputBluePrint},
        props::{BoxBlueprint, PropBlueprint},
        wall::WallBluePrint,
    },
    gate::*,
    input::{ButtonType, Input, InputPlugin},
    level::Level,
    logic_tree::*,
    output::{Output, OutputPlugin},
};

pub struct LevelManagerPlugin;
impl LevelManagerPlugin {
    pub fn gen_blueprint() -> Blueprint {
        let walls = vec![
            WallBluePrint::new(vec2(0.0, -50.0), 0.0, vec2(500.0, 5.0)),
            WallBluePrint::new(vec2(0.0, 50.0), 0.0, vec2(500.0, 5.0)),
            WallBluePrint::new(vec2(250.0, 0.0), 0.0, vec2(5.0, 100.0)),
            WallBluePrint::new(vec2(-250.0, 0.0), 0.0, vec2(5.0, 100.0)),
        ];

        let props = vec![PropBlueprint::BoxBlueprint(BoxBlueprint::new(
            vec2(-100.0, 0.0),
            0.0,
            vec2(5.0, 5.0),
        ))];

        let inputs = vec![
            InputBlueprint::Button(ButtonBlueprint::new(
                vec2(-50.0, -30.0),
                5.0,
                ButtonType::PressButton,
            )),
            InputBlueprint::Button(ButtonBlueprint::new(
                vec2(-50.0, 30.0),
                5.0,
                ButtonType::ToggleButton,
            )),
        ];

        let outputs = vec![OutputBluePrint::Door(DoorBlueprint::new(
            vec2(50.0, 0.0),
            0.0,
            vec2(5.0, 100.0),
        ))];

        let logic_tree = LogicTree::new(
            vec![vec![GateTypes::OrGate(OrGate::default())]],
            vec![vec![0, 1], vec![0]],
        );

        let bp = Blueprint {
            walls,
            props,
            inputs,
            outputs: outputs,
            logic_tree,
        };

        return bp;
    }
}
impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((InputPlugin, OutputPlugin));

        app.insert_resource(LevelManager { cur_level: None })
            .add_systems(Startup, load_level)
            .add_systems(Update, update_level);
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

        level_manager.cur_level = Some(blueprint.spawn(&mut commands, &handles));
    } else {
        level_manager.cur_level = Some(
            Blueprint::load_cfg(&format!("{}/{}", level_config.dir, level_config.cur_file))
                .spawn(&mut commands, &handles),
        );
    }
}

pub fn update_level(
    mut level_manager: ResMut<LevelManager>,
    inputs: Query<One<&dyn Input>>,
    mut outputs: Query<One<&mut dyn Output>>,
) {
    if let Some(level) = &mut level_manager.cur_level {
        level.update_state(&inputs, &mut outputs);
    }
}

#[derive(Resource)]
pub struct LevelManager {
    cur_level: Option<Level>,
}
