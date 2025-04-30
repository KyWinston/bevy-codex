use crate::SimulationState;

use bevy::prelude::*;
use components::PauseMenu;
use systems::{go_to_paused, register_pause_actions};

pub mod components;
mod systems;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_pause_actions)
            .add_systems(OnEnter(SimulationState::Paused), go_to_paused)
            .add_systems(
                OnExit(SimulationState::Paused),
                |mut commands: Commands, node: Query<Entity, With<PauseMenu>>| {
                    if let Ok(node) = node.single() {
                        commands.entity(node).despawn();
                    }
                },
            );
    }
}
