use bevy::{
    color::Color,
    prelude::{
        default, BuildChildren, Bundle, ButtonBundle, ChildBuilder, Component, Entity, TextBundle,
    },
    text::{Text, TextStyle},
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

    pub fn spawn(self, commands: &mut ChildBuilder, text: String) -> Entity {
        return commands
            .spawn(self)
            .with_children(|child_builder| {
                child_builder.spawn(TextBundle {
                    text: Text::from_section(
                        text,
                        TextStyle {
                            font_size: 12.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ),
                    style: Style {
                        // justify_content: JustifyContent::SpaceAround,
                        ..default()
                    },
                    ..default()
                });
            })
            .id();
    }
}

#[derive(Component)]
pub enum ButtonEvent {
    LevelPack(LoadLevelPackEvent),
    Level(LoadLevelEvent),
}
impl ButtonEvent {
    pub fn new_level_pack(level_pack_dir: &str) -> Self {
        return Self::LevelPack(LoadLevelPackEvent::new(level_pack_dir));
    }

    pub fn new_level(id: usize) -> Self {
        return Self::Level(LoadLevelEvent::new(id));
    }
}
