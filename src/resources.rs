use bevy::{prelude::*, window::SystemCursorIcon};

#[derive(Asset, TypePath, Resource, Clone)]
pub struct CodexSettings {
    pub title: String,
}

#[derive(Resource)]
pub struct GameSettingsFolder(pub String);

impl Default for CodexSettings {
    fn default() -> Self {
        Self {
            title: "test".to_string(),
        }
    }
}
impl CodexSettings {
    pub fn new(title: String) -> Self {
        Self { title, ..default() }
    }
}

#[derive(Resource)]
pub struct CursorIcons(pub Vec<SystemCursorIcon>);
