use bevy::prelude::{Bundle, TextBundle};

#[derive(Bundle)]

pub struct Text {
    text_bundle: TextBundle,
}
impl Text {
    pub fn new(text: Vec<&str>) -> Self {
        return Self {
            text_bundle: TextBundle::from_section(),
        };
    }
}
