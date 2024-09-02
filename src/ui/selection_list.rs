use bevy::{
    a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    },
    prelude::{
        default, BuildChildren, Bundle, Commands, Component, Entity, Event, Events, NodeBundle,
    },
    ui::{AlignItems, FlexDirection, JustifyContent, Style, Val},
};

use super::events::{LoadLevelEvent, LoadLevelPackEvent};

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
    fn get_text(&self) -> Vec<String>;
}

pub struct LevelPackItem {
    name: String,
    progress: f32,
    rating: f32,
}
impl LevelPackItem {
    pub fn new(name: &str, progress: f32, rating: f32) -> Self {
        return Self {
            name: name.to_string(),
            progress,
            rating,
        };
    }
}
impl ListItem for LevelPackItem {
    fn get_text(&self) -> Vec<String> {
        return vec![
            self.name.clone(),
            self.progress.to_string(),
            self.rating.to_string(),
        ];
    }
}

pub struct LevelItem {
    name: ElementType,
    complete: ElementType,
}
impl LevelItem {
    pub fn new(name: &str, complete: bool) -> Self {
        return Self {
            name: ElementType::Text(name.to_string()),
            complete,
        };
    }
}

enum ElementType {
    Text(String),
    Button(String),
}
pub enum ButtonEvent(){
    LevelPack(LoadLevelPackEvent),
    Level(LoadLevelEvent)
}
