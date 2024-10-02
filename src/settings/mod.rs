use self::systems::build_settings;
use bevy::prelude::*;
use bevy_lunex::UiGenericPlugins;
use components::SettingsPgUi;

pub mod components;
pub mod events;
pub mod styles;
pub mod systems;

pub struct SettingsUiPlugin;

impl Plugin for SettingsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiGenericPlugins::<SettingsPgUi>::new(),
        ))
        .add_systems(Update, build_settings);
    }
}
