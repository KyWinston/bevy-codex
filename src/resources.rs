use bevy::{color::palettes::css::WHITE, prelude::*, utils::HashMap};
use serde::Deserialize;

#[derive(Asset, TypePath, Resource, Clone)]
pub struct CodexSettings {
    pub title: String,
    pub button_texture: String,
    pub button_color: Color,
    pub font: String,
    pub debug: bool,
    pub splash_duration: f32,
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
            splash_duration: 5.0,
        }
    }
}
