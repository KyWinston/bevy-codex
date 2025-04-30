use bevy::prelude::*;
use components::Loading;
use systems::load_game_screen;

use crate::UiState;

pub mod components;
mod systems;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::Loading), load_game_screen)
            .add_systems(
                OnExit(UiState::Loading),
                |mut commands: Commands, node: Query<Entity, With<Loading>>| {
                    for node in node.iter() {
                        commands.entity(node).despawn();
                    }
                },
            );
    }
}
