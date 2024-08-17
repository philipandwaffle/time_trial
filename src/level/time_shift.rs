use bevy::{
    app::{Plugin, Update},
    input::ButtonInput,
    math::Vec2,
    prelude::{
        Component, Event, EventReader, EventWriter, KeyCode, Query, Res, ResMut, Resource,
        Transform,
    },
};

use crate::configuration::key_bindings::KeyBinds;

pub struct TimeShiftPlugin;
impl Plugin for TimeShiftPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(TimeState::default())
            .add_event::<TimeShiftEvent>()
            .add_systems(Update, (read_time_shift_events, write_time_shift_events));
    }
}

#[derive(Component)]
pub struct TimeShift {
    past_pos: Vec2,
}
impl TimeShift {
    pub fn new(cur_pos: Vec2) -> Self {
        return Self { past_pos: cur_pos };
    }
}

#[derive(Resource)]
pub struct TimeState {
    pub is_present: bool,
}
impl Default for TimeState {
    fn default() -> Self {
        Self { is_present: false }
    }
}

#[derive(Event)]
pub struct TimeShiftEvent;

fn write_time_shift_events(
    mut time_shift_ev: EventWriter<TimeShiftEvent>,
    keys: Res<ButtonInput<KeyCode>>,
    key_binds: Res<KeyBinds>,
) {
    if keys.just_pressed(key_binds.time_shift.0) {
        time_shift_ev.send(TimeShiftEvent);
    }
}
fn read_time_shift_events(
    mut time_state: ResMut<TimeState>,
    mut time_shift_ev: EventReader<TimeShiftEvent>,
    mut ents: Query<(&mut Transform, &mut TimeShift)>,
) {
    for _ in time_shift_ev.read() {
        match time_state.is_present {
            true => {
                for (mut transform, time_shift) in ents.iter_mut() {
                    let z = transform.translation.z;
                    transform.translation = time_shift.past_pos.extend(z);
                }
            }
            false => {
                for (transform, mut time_shift) in ents.iter_mut() {
                    time_shift.past_pos = transform.translation.truncate();
                }
            }
        }
        time_state.is_present = !time_state.is_present;
    }
}
