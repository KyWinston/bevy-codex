use bevy::{
    color::palettes::css::WHITE,
    log::{Level, LogPlugin},
    prelude::*,
};

use bevy_codex::{
    prelude::{UiScreensPlugin, UiState},
    resources::CodexSettings,
};
use systems::{move_to_hud, run_dialog, setup, start_load};

pub mod resources;
pub mod systems;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::INFO,
                    filter: "bevy_midi=debug".to_string(),
                    ..default()
                })
                .build(),
            UiScreensPlugin {
                game_settings_folder: "".to_string(),
                config: CodexSettings::new("dialogue".to_string(), None, WHITE.into()),
            },
        ))
        .add_systems(Startup, setup)
        .add_systems(OnEnter(UiState::Loading), start_load)
        .add_systems(
            Update,
            (run_dialog, move_to_hud.run_if(in_state(UiState::Loading))),
        )
        .run();
}
