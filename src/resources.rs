use bevy::{color::palettes::css::WHITE, prelude::*, utils::HashMap};
use serde::Deserialize;

#[derive(Asset, TypePath, Resource, Clone)]
pub struct CodexSettings {
    pub title: String,
    pub button_texture: String,
    pub button_color: Color,
    pub font: String,
    pub debug: bool,
    pub splash_pages: Vec<SplashPage>,
}

#[derive(Clone)]
pub struct SplashPage {
    pub(crate) splash_header: String,
    pub(crate) splash_logo: String,
    pub(crate) splash_footer: String,
    pub(crate) splash_duration: f32,
}

impl Default for SplashPage {
    fn default() -> Self {
        Self {
            splash_header: "".to_string(),
            splash_logo: "icon".to_string(),
            splash_footer: "".to_string(),
            splash_duration: 5.0,
        }
    }
}

impl SplashPage {
    pub fn new(
        splash_header: &str,
        splash_logo: &str,
        splash_footer: &str,
        splash_duration: f32,
    ) -> Self {
        Self {
            splash_header: splash_header.to_string(),
            splash_logo: splash_logo.to_string(),
            splash_footer: splash_footer.to_string(),
            splash_duration,
        }
    }
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
            splash_pages: vec![SplashPage::default()],
        }
    }
}
impl CodexSettings {
    pub fn new(title: String, button_texture: Option<String>, button_color: Color) -> Self {
        Self {
            title,
            button_texture:button_texture.unwrap_or_default(),
            button_color,
            ..default()
        }
    }
    pub fn with_splash(&mut self, splash_pages: Vec<SplashPage>) -> Self {
        self.splash_pages = splash_pages;
        self.clone()
    }
}
