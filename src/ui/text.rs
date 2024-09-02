use bevy::{
    color::Color,
    prelude::{default, Bundle, TextBundle},
    text::{TextSection, TextStyle},
};

#[derive(Bundle)]

pub struct Text {
    text_bundle: TextBundle,
}
impl Text {
    pub fn new(text: Vec<&str>, font_size: f32, color: Color) -> Self {
        let style = TextStyle {
            font_size,
            color,
            ..default()
        };

        return Self {
            text_bundle: TextBundle::from_sections(
                text.iter().map(|x| TextSection::new(*x, style.clone())),
            ),
        };
    }
}
