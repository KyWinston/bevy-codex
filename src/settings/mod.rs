use crate::UiState;

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_hui_widgets::prelude::SliderChangedEvent;
use components::SettingsPg;
use resources::GameSettings;
use systems::{go_to_settings, register_settings_actions, update_settings};

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;
pub struct SettingsUiPlugin;

impl Plugin for SettingsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RonAssetPlugin::<GameSettings>::new(&["settings.ron"]),))
            .add_systems(Startup, register_settings_actions)
            .add_systems(OnEnter(UiState::Settings), go_to_settings)
            .add_systems(
                Update,
                update_settings.run_if(on_event::<SliderChangedEvent>),
            )
            .add_systems(
                OnExit(UiState::Settings),
                |mut commands: Commands, node: Query<Entity, With<SettingsPg>>| {
                    if let Ok(node) = node.single() {
                        commands.entity(node).despawn();
                    }
                },
            );
    }
}
