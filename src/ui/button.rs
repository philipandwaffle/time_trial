use bevy::{
    prelude::{default, Bundle, ButtonBundle, ChildBuilder, Component, Entity},
    ui::{AlignItems, JustifyContent, Style, Val},
};

use super::events::{LoadLevelEvent, LoadLevelPackEvent};

#[derive(Bundle)]
pub struct EventButtonBundle {
    button_bundle: ButtonBundle,
    button_event: ButtonEvent,
}
impl EventButtonBundle {
    pub fn new(width: f32, height: f32, event: ButtonEvent) -> Self {
        return Self {
            button_bundle: ButtonBundle {
                style: Style {
                    width: Val::Percent(width),
                    height: Val::Percent(height),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            button_event: event,
        };
    }

    pub fn spawn(self, commands: &mut ChildBuilder) -> Entity {
        return commands.spawn(self).id();
    }
}

#[derive(Component)]
pub enum ButtonEvent {
    LevelPack(LoadLevelPackEvent),
    Level(LoadLevelEvent),
}
