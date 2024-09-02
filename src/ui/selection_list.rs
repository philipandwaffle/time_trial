use std::process::id;

use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    color::Color,
    prelude::{
        default, BuildChildren, Bundle, ChildBuilder, Commands, Component, Entity, Event, Events,
        NodeBundle, TextBundle,
    },
    text::TextStyle,
    ui::{AlignItems, FlexDirection, JustifyContent, Style, Val},
};

use super::{
    button::{ButtonEvent, EventButtonBundle},
    events::{LoadLevelEvent, LoadLevelPackEvent},
};

#[derive(Component)]
pub struct UIList;

#[derive(Bundle)]
pub struct UIListBundle {
    marker: UIList,
    node_bundle: NodeBundle,
    accessibility_node: AccessibilityNode,
}
impl UIListBundle {
    pub fn new() -> Self {
        return Self {
            marker: UIList,
            node_bundle: NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            accessibility_node: AccessibilityNode(NodeBuilder::new(Role::List)),
        };
    }
    pub fn spawn(self, commands: &mut Commands) -> Entity {
        let root_ent = commands.spawn(self).id();
        // commands.get_entity(root_ent).unwrap().with_children(|p|{
        //     for
        // })
        return root_ent;
    }
}

// #[derive(Component)]
// pub struct UIList<T: Sync + Send> {
//     items: Vec<T>,
// }
// impl<T: Sync + Send> UIList<T> {
//     pub fn new(items: Vec<T>) -> Self {
//         return Self { items };
//     }
// }

pub trait ListItem {
    fn spawn(self, commands: &mut ChildBuilder) -> Entity;
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
            progress: ElementType::Text(progress.to_string()),
            rating: ElementType::Text(format!("{rating}%")),
            load_level_pack: ElementType::Button(ButtonEvent::new_level_pack(level_pack_dir)),
        };
    }
}
impl ListItem for LevelPackItem {
    fn spawn(self, commands: &mut ChildBuilder) -> Entity {
        return commands
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|node| {
                self.name.spawn(node, 0.25);
                self.progress.spawn(node, 0.25);
                self.rating.spawn(node, 0.25);
                self.load_level_pack.spawn(node, 0.25);
            }).id();
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
            load_level: ElementType::Button(ButtonEvent::new_level(level_id)),
        };
    }
}

enum ElementType {
    Text(String),
    Button(ButtonEvent),
}
impl ElementType {
    fn spawn(self, commands: &mut ChildBuilder, width: f32) -> Entity {
        return match self {
            ElementType::Text(val) => commands
                .spawn(TextBundle::from_section(
                    val,
                    TextStyle {
                        font_size: 12.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ))
                .id(),
            ElementType::Button(event) => commands
                .spawn(EventButtonBundle::new(width, 100.0, event))
                .id(),
        };
    }
}
