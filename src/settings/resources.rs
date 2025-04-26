use bevy::{platform::collections::HashMap, prelude::*};
use serde::Deserialize;

#[derive(Resource)]
pub struct SettingsHandle(pub Handle<GameSettings>);

#[derive(Asset, TypePath, Deserialize)]
pub struct GameSettings {
    pub audio_settings: HashMap<String, f32>,
    pub network_settings: Vec<HashMap<String, String>>,
}

pub enum SettingType {
    Text(String),
    Number(f32),
}

impl GameSettings {
    pub fn change_setting(
        &mut self,
        target_key: String,
        new_value: SettingType,
        player_id: Option<usize>,
    ) {
        for (key, value) in self.audio_settings.iter_mut() {
            if target_key == *key {
                match new_value {
                    SettingType::Text(_) => {}
                    SettingType::Number(n) => {
                        *value = n;
                    }
                }
            }
        }
        if player_id.is_some() {
            for (key, value) in self.network_settings[player_id.unwrap()].iter_mut() {
                if target_key == *key {
                    match new_value {
                        SettingType::Text(ref s) => *value = s.to_string(),
                        SettingType::Number(_) => {}
                    }
                }
            }
        }
    }
    pub fn get_setting(&self, target_key: String, player_id: Option<usize>) -> Option<SettingType> {
        for (key, value) in self.audio_settings.iter() {
            if target_key == *key {
                return Some(SettingType::Number(*value));
            } else {
                continue;
            }
        }
        if player_id.is_some() {
            for (key, value) in self.network_settings[player_id.unwrap()].iter() {
                if target_key == *key {
                    return Some(SettingType::Text(value.to_string()));
                } else {
                    continue;
                }
            }
        }
        None
    }
}
