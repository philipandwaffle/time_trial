use bevy::{
    prelude::{default, BuildChildren, Commands, Entity, NodeBundle},
    ui::{JustifyContent, Style, Val},
};

use super::selection_list::{LevelPackItem, UIListBundle};

pub struct UIRoots {
    main_menu: Entity,
    level_packs: Entity,
    level_pack: Entity,
    level: Entity,
}
impl UIRoots {
    pub fn new(commands: &mut Commands) -> Self {
        let mut me = Self {
            main_menu: Entity::PLACEHOLDER,
            level_packs: Entity::PLACEHOLDER,
            level_pack: Entity::PLACEHOLDER,
            level: Entity::PLACEHOLDER,
        };
        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            })
            .with_children(|cb| {
                // me.level_packs =
                //     UIListBundle::spawn(cb, Box::new(LevelPackItem::new_title()), vec![]);
                // me.level_pack = UIListBundle::spawn(cb, vec![]);
            });

        return me;
    }
}
