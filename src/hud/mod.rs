use bevy::prelude::*;
use systems::open_hud;

use crate::SimulationState;

pub mod components;
pub mod systems;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SimulationState::Running), open_hud);
    }
}
