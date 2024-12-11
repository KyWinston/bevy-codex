use bevy::prelude::*;
use components::MainMenu;
use systems::{go_to_main, register_menu_actions};

use crate::UiState;

pub mod components;
pub mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_menu_actions)
            .add_systems(OnEnter(UiState::MainMenu), go_to_main)
            .add_systems(
                OnExit(UiState::MainMenu),
                |mut commands: Commands, node: Query<Entity, With<MainMenu>>| {
                    if let Ok(node) = node.get_single() {
                        commands.entity(node).despawn_recursive();
                    }
                },
            );
    }
}
