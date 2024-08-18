use std::collections::HashMap;

use bevy::{
    app::{Plugin, PostUpdate, Startup},
    asset::Assets,
    math::vec2,
    prelude::{Commands, Query, Res, ResMut, Resource},
    sprite::ColorMaterial,
};
use bevy_trait_query::One;

use crate::{
    configuration::{level::LevelConfig, material::HSL, Config},
    handles::Handles,
};

use super::{
    blueprints::{
        input::{ButtonBlueprint, InputBlueprint},
        level::{LevelBlueprint, LevelMaterialHandles},
        output::{DoorBlueprint, OutputBluePrint},
        props::{BoxBlueprint, PropBlueprint},
        wall::WallBluePrint,
    },
    gate::*,
    input::{ButtonType, Input, InputPlugin},
    level::Level,
    level_pack::LevelPackPlugin,
    logic_tree::*,
    output::{Output, OutputPlugin},
    time_shift::TimeShiftPlugin,
};

pub struct LevelManagerPlugin;
impl LevelManagerPlugin {
    pub fn gen_blueprint() -> LevelBlueprint {
        let walls = vec![
            WallBluePrint::new(vec2(0.0, -50.0), 0.0, vec2(500.0, 5.0), "wall"),
            WallBluePrint::new(vec2(0.0, 50.0), 0.0, vec2(500.0, 5.0), "wall"),
            WallBluePrint::new(vec2(250.0, 0.0), 0.0, vec2(5.0, 100.0), "wall"),
            WallBluePrint::new(vec2(-250.0, 0.0), 0.0, vec2(5.0, 100.0), "wall"),
        ];

        let props = vec![PropBlueprint::BoxBlueprint(BoxBlueprint::new(
            vec2(-100.0, 0.0),
            0.0,
            vec2(20.0, 20.0),
            "box",
        ))];

        let inputs = vec![
            InputBlueprint::Button(ButtonBlueprint::new((
                vec2(-50.0, -30.0),
                10.0,
                ButtonType::PressButton,
                "green_on",
                "green_off",
            ))),
            InputBlueprint::Button(ButtonBlueprint::new((
                vec2(-50.0, 30.0),
                10.0,
                ButtonType::ToggleButton,
                "yellow_on",
                "yellow_off",
            ))),
        ];

        let outputs = vec![OutputBluePrint::Door(DoorBlueprint::new(
            vec2(50.0, 0.0),
            0.0,
            vec2(5.0, 100.0),
            "door",
        ))];

        let logic_tree = LogicTree::new(
            vec![vec![GateTypes::OrGate(OrGate::default())]],
            vec![vec![0, 1], vec![0]],
        );

        let mut level_materials = HashMap::<String, HSL>::new();
        level_materials.insert("wall".to_string(), HSL::new(0.0, 0.0, 0.0));
        level_materials.insert("box".to_string(), HSL::new(230.0, 0.5, 0.5));
        level_materials.insert("green_on".to_string(), HSL::new(110.0, 0.5, 0.5));
        level_materials.insert("green_off".to_string(), HSL::new(110.0, 0.5, 0.3));
        level_materials.insert("yellow_on".to_string(), HSL::new(67.0, 0.5, 0.5));
        level_materials.insert("yellow_off".to_string(), HSL::new(67.0, 0.5, 0.3));
        level_materials.insert("door".to_string(), HSL::new(0.0, 0.0, 0.5));

        let bp = LevelBlueprint::new(walls, props, inputs, outputs, logic_tree, level_materials);

        return bp;
    }
}
impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((InputPlugin, OutputPlugin, LevelPackPlugin, TimeShiftPlugin));

        app.insert_resource(LevelManager { cur_level: None })
            .insert_resource(LevelMaterialHandles::default())
            .add_systems(Startup, gen_level)
            .add_systems(PostUpdate, update_level_state);
    }
}

fn gen_level(level_config: Res<LevelConfig>) {
    if level_config.gen_on_start {
        let blueprint = LevelManagerPlugin::gen_blueprint();
        if level_config.save_on_start {
            blueprint.save_cfg(&format!("{}/{}", level_config.dir, level_config.cur_level));
        }
    }
}

fn update_level_state(
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
impl LevelManager {
    pub fn change_level(
        &mut self,
        blueprint: LevelBlueprint,
        commands: &mut Commands,
        handles: &Handles,
        level_material_handles: &mut LevelMaterialHandles,
        materials: &mut Assets<ColorMaterial>,
    ) {
        if let Some(level) = &self.cur_level {
            level.despawn(commands);
        }

        self.cur_level =
            Some(blueprint.spawn(commands, handles, level_material_handles, materials));
    }
}
