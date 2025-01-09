use crate::UiState;

use bevy::prelude::*;
use components::SettingsPg;
use systems::{go_to_settings, register_settings_actions};

pub mod components;
pub mod events;
pub mod systems;

pub struct SettingsUiPlugin;

impl Plugin for SettingsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_settings_actions)
            .add_systems(OnEnter(UiState::Settings), go_to_settings)
            .add_systems(
                OnExit(UiState::Settings),
                |mut commands: Commands, node: Query<Entity, With<SettingsPg>>| {
                    if let Ok(node) = node.get_single() {
                        commands.entity(node).despawn_recursive();
                    }
                },
            );
    }
}
