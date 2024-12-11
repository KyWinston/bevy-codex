use bevy::prelude::*;
use components::Hud;
use systems::open_hud;

use crate::UiState;

pub mod components;
pub mod systems;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(UiState::Hud), open_hud)
        .add_systems(
            OnExit(UiState::Hud),
            |mut commands: Commands, node: Query<Entity, With<Hud>>| {
                for node in node.iter() {
                    commands.entity(node).despawn_recursive();
                }
            },
        );
    }
}
