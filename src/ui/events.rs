use bevy::{app::Plugin, prelude::Event};

pub struct UIEventPlugin;
impl Plugin for UIEventPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<LoadLevelPackEvent>()
            .add_event::<LoadLevelEvent>();
    }
}

#[derive(Event)]
pub struct LoadLevelPackEvent {
    pub dir: String,
}
impl LoadLevelPackEvent {
    pub fn new(dir: &str) -> Self {
        return Self {
            dir: dir.to_string(),
        };
    }
}

#[derive(Event)]
pub struct LoadLevelEvent {
    pub id: usize,
}
impl LoadLevelEvent {
    pub fn new(id: usize) -> Self {
        return Self { id };
    }
}
