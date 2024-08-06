use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::UiState;

use self::{
    resources::SettingsVals,
    systems::{build_settings, load_settings_toml},
};

pub mod components;
pub mod events;
pub mod resources;
pub mod styles;
pub mod systems;

#[derive(Resource, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "bevy_settings::serde")]
struct Settings {
    master_volume: f64,
    custom_cursor: bool,
}
pub struct SettingsUiPlugin;

impl Plugin for SettingsUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<SettingsVals>(SettingsVals(vec![]))
            .add_plugins(bevy_settings::SettingsPlugin::<Settings>::new(
                "GkPixel",
                "Dreamlighters",
            ))
            .add_systems(OnEnter(UiState::Settings), build_settings);
    }
}
