use bevy::prelude::*;
use bevy_lunex::UiSystems;
use systems::build_hud;


pub mod components;
pub mod systems;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, build_hud.before(UiSystems::Compute));
    }
}
