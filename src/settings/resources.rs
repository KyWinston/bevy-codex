use bevy::{prelude::*, utils::HashMap};
use serde::Deserialize;


#[derive(Resource)]
pub struct SettingsHandle(pub Handle<GameSettings>);

#[derive(Asset, TypePath, Deserialize)]
pub struct GameSettings {
    pub audio_settings: HashMap<String, f32>,
    pub network_port: Option<u32>,
}