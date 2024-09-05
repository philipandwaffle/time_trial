use std::process::id;

use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    app::{Plugin, Update},
    color::Color,
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::{
        default, BuildChildren, Bundle, ChildBuilder, Commands, Component, Entity, Event,
        EventReader, Events, NodeBundle, Parent, Query, TextBundle,
    },
    text::{Text, TextStyle},
    ui::{
        AlignContent, AlignItems, AlignSelf, FlexDirection, JustifyContent, Node, Overflow, Style,
        Val,
    },
};

use super::{
    button::{ButtonEvent, EventButtonBundle},
    events::{LoadLevelEvent, LoadLevelPackEvent},
};

pub struct ScrollingListPlugin;
impl Plugin for ScrollingListPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, mouse_scroll);
    }
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, Entity, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, ent, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            // let container_height = query_node.get(ent).unwrap().size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-1000.0, 0.);
            style.top = Val::Px(scrolling_list.position);
            println!(
                "{}, {}, {}, {}",
                scrolling_list.position,
                list_node.size(),
                max_scroll,
                container_height
            );
        }
    }
}

#[derive(Component, Default)]
pub struct ScrollingList {
    position: f32,
}

#[derive(Bundle)]
pub struct UIListBundle {
    scrolling_list: ScrollingList,
    node_bundle: NodeBundle,
    accessibility_node: AccessibilityNode,
}
impl UIListBundle {
    pub fn new() -> Self {
        return Self {
            scrolling_list: ScrollingList::default(),
            node_bundle: NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    overflow: Overflow::clip_y(),
                    align_self: AlignSelf::Stretch,
                    height: Val::Percent(50.),
                    width: Val::Percent(100.),
                    ..default()
                },
                background_color: Color::hsla(200.0, 0.5, 0.5, 0.1).into(),
                ..default()
            },
            accessibility_node: AccessibilityNode(NodeBuilder::new(Role::List)),
        };
    }
    pub fn spawn(self, child_builder: &mut ChildBuilder, items: Vec<Box<dyn ListItem>>) {
        child_builder.spawn(self).with_children(|mut p| {
            for item in items {
                item.spawn(&mut p);
            }
        });
    }
}

pub trait ListItem {
    fn spawn(self: Box<Self>, child_builder: &mut ChildBuilder) -> Entity;
}

pub struct LevelPackItem {
    name: ElementType,
    progress: ElementType,
    rating: ElementType,
    load_level_pack: ElementType,
}
impl LevelPackItem {
    pub fn new(name: &str, progress: f32, rating: f32, level_pack_dir: &str) -> Self {
        return Self {
            name: ElementType::Text(name.to_string()),
            progress: ElementType::Text(format!("{progress}%")),
            rating: ElementType::Text(format!("{rating}")),
            load_level_pack: ElementType::Button(
                ButtonEvent::new_level_pack(level_pack_dir),
                "Load Pack".to_string(),
            ),
        };
    }
}
impl ListItem for LevelPackItem {
    fn spawn(self: Box<LevelPackItem>, child_builder: &mut ChildBuilder) -> Entity {
        return child_builder
            .spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        width: Val::Percent(100.0),
                        ..default()
                    },

                    ..default()
                },
                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
            ))
            .with_children(|row| {
                self.name.spawn(row, 25.0);
                self.progress.spawn(row, 25.0);
                self.rating.spawn(row, 25.0);
                self.load_level_pack.spawn(row, 25.0);
            })
            .id();
    }
}

pub struct LevelItem {
    name: ElementType,
    complete: ElementType,
    load_level: ElementType,
}
impl LevelItem {
    pub fn new(name: &str, complete: bool, level_id: usize) -> Self {
        return Self {
            name: ElementType::Text(name.to_string()),
            complete: ElementType::Text(complete.to_string()),
            load_level: ElementType::Button(
                ButtonEvent::new_level(level_id),
                "Load Level".to_string(),
            ),
        };
    }
}

enum ElementType {
    Text(String),
    Button(ButtonEvent, String),
}
impl ElementType {
    fn spawn(self, child_builder: &mut ChildBuilder, width: f32) {
        match self {
            ElementType::Text(val) => {
                child_builder
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(width),
                            // align_content: AlignContent::Stretch,
                            justify_content: JustifyContent::SpaceAround,
                            ..default()
                        },
                        background_color: Color::hsla(200.0, 0.5, 0.8, 0.1).into(),
                        ..default()
                    })
                    .with_children(|child_builder| {
                        child_builder.spawn(TextBundle {
                            text: Text::from_section(
                                val,
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
                    });
            }
            ElementType::Button(event, text) => {
                EventButtonBundle::new(width, 100.0, event).spawn(child_builder, text);
            }
        }
    }
}
