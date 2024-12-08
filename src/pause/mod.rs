use crate::SimulationState;

use bevy::prelude::*;
use bevy_hui::prelude::HtmlNode;
use systems::{go_to_paused, register_pause_actions};

pub mod components;
mod systems;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(SimulationState::Paused),
            (go_to_paused, register_pause_actions),
        )
        .add_systems(
            OnExit(SimulationState::Paused),
            |mut commands: Commands, node: Query<Entity, With<HtmlNode>>| {
                for node in node.iter() {
                    commands.entity(node).despawn_recursive();
                }
            },
        );
    }
}
