use bevy::app::Plugin;

use super::{button::ButtonPlugin, selection_list::ScrollingListPlugin};

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((ButtonPlugin, ScrollingListPlugin));
    }
}
