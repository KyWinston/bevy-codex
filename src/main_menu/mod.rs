use bevy::prelude::*;
use components::MainMenu;
use systems::{go_to_main, register_menu_actions};

use crate::UiState;

pub mod components;
pub mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(UiState::MainMenu),
            (register_menu_actions, go_to_main),
        )
        .add_systems(
            OnExit(UiState::MainMenu),
            |mut commands: Commands, node: Query<Entity, With<MainMenu>>| {
                for node in node.iter() {
                    commands.entity(node).despawn_recursive();
                }
            },
        );
    }
}
