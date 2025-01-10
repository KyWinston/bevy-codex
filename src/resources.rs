use bevy::{color::palettes::css::WHITE, prelude::*, utils::HashMap, window::SystemCursorIcon};
use serde::Deserialize;

#[derive(Asset, TypePath, Resource, Clone)]
pub struct CodexSettings {
    pub title: String,
    pub button_texture: String,
    pub button_color: Color,
    pub font: String,
    pub debug: bool,
}

#[derive(Asset, TypePath, Resource, Clone, Deserialize)]
pub struct GameSettings {
    pub audio_settings: HashMap<String, f32>,
    pub network_port: Option<u32>,
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
impl CodexSettings {
    pub fn new(title: String, button_texture: Option<String>, button_color: Color) -> Self {
        Self {
            title,
            button_texture: button_texture.unwrap_or_default(),
            button_color,
            ..default()
        }
    }
}

#[derive(Resource)]
pub struct CursorIcons(pub Vec<SystemCursorIcon>);


