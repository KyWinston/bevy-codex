use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct CodexSettings {
    pub title: String,
    pub button_texture: String,
    pub font: String,
    pub debug: bool,
}

impl Default for CodexSettings {
    fn default() -> Self {
        Self {
            title: "test".to_string(),
            button_texture: "tile_0003.png".to_string(),
            font: "fonts/FiraSans-Bold.ttf".to_string(),
            debug: false,
        }
    }
}
