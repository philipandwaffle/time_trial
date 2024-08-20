use bevy::{
    app::{Plugin, Update},
    prelude::{Entity, EventWriter, Query, Res, With},
};
use bevy_rapier2d::plugin::RapierContext;

use crate::player::player_bundle::Player;

use super::{bundles::goal::Goal, level_pack::ChangeLevelEvent};

pub struct GoalPlugin;
impl Plugin for GoalPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, update_goal);
    }
}

fn update_goal(
    mut goals: Query<(&mut Goal, Entity)>,
    player: Query<Entity, With<Player>>,
    rapier_context: Res<RapierContext>,
    mut ev_change_level: EventWriter<ChangeLevelEvent>,
) {
    if let Ok(player_ent) = player.get_single() {
        if let Ok((mut goal, goal_ent)) = goals.get_single_mut() {
            if goal.triggered {
                return;
            }
            if let Some(intersect) = rapier_context.intersection_pair(goal_ent, player_ent) {
                if intersect {
                    goal.triggered = true;
                    ev_change_level.send(ChangeLevelEvent::new(1));
                }
            }
        }
    }
}
