use bevy::{color::palettes::css::WHITE, prelude::*};

#[derive(Resource, Clone)]
pub struct CodexSettings {
    pub title: String,
    pub button_texture: String,
    pub button_color: Color,
    pub font: String,
    pub debug: bool,
}

impl Default for CodexSettings {
    fn default() -> Self {
        Self {
            title: "test".to_string(),
            button_texture: "tile_0003.png".to_string(),
            button_color: WHITE.into(),
            font: "fonts/FiraSans-Bold.ttf".to_string(),
            debug: false,
        }
    }
}
