use bevy::{
    app::{Plugin, Update},
    color::Color,
    math::vec3,
    prelude::{default, Bundle, Component, Event, EventReader, Query, Res, Transform, Visibility},
    text::{Text, Text2dBundle, TextSection, TextStyle},
    time::{Stopwatch, Time},
};

use crate::consts::TEXT_SCALE;
pub struct StopwatchPlugin;
impl Plugin for StopwatchPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                reset_stopwatch,
                update_stopwatch_timer,
                update_stopwatch_display,
            ),
        );
    }
}

#[derive(Bundle)]
pub struct TimerBundle {
    stopwatch_display: StopwatchDisplay,
    text_2d_bundle: Text2dBundle,
}
impl TimerBundle {
    pub fn new(font_size: f32) -> Self {
        return Self {
            stopwatch_display: StopwatchDisplay::new(),
            text_2d_bundle: Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font_size,
                            color: Color::BLACK,
                            ..default()
                        },
                    }],
                    ..default()
                },
                transform: Transform {
                    translation: vec3(0.0, 0.0, 0.05),
                    scale: TEXT_SCALE,
                    ..default()
                },
                visibility: Visibility::Visible,
                ..default()
            },
        };
    }
}

#[derive(Component)]
pub struct StopwatchDisplay {
    time: Stopwatch,
}
impl StopwatchDisplay {
    pub fn new() -> Self {
        return Self {
            time: Stopwatch::new(),
        };
    }
}

#[derive(Event)]
pub struct ResetStopwatchEvent;

pub fn reset_stopwatch(
    mut reset_stopwatch_ev: EventReader<ResetStopwatchEvent>,
    mut stopwatches: Query<&mut StopwatchDisplay>,
) {
    if reset_stopwatch_ev.is_empty() {
        return;
    }

    for mut stopwatch in stopwatches.iter_mut() {
        stopwatch.time.reset();
    }

    reset_stopwatch_ev.clear()
}

pub fn update_stopwatch_timer(time: Res<Time>, mut stopwatches: Query<&mut StopwatchDisplay>) {
    for mut stopwatch in stopwatches.iter_mut() {
        stopwatch.time.tick(time.delta());
    }
}

pub fn update_stopwatch_display(mut stopwatches: Query<(&mut Text, &StopwatchDisplay)>) {
    for (mut text, stopwatch_display) in stopwatches.iter_mut() {
        let duration = stopwatch_display.time.elapsed();
        text.sections[0].value = format!("{}:{}", duration.as_secs(), duration.subsec_millis());
    }
}
