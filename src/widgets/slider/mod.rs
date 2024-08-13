use bevy::prelude::*;
use bevy_lunex::prelude::*;
use components::SliderUi;
use systems::build_slider;

use crate::resources::CodexSettings;

pub mod components;
pub mod systems;

#[derive(Clone)]
pub struct SliderPlugin;

impl Plugin for SliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiGenericPlugin::<SliderUi>::new());
        if app.world().resource::<CodexSettings>().debug {
            app.add_plugins(UiDebugPlugin::<SliderUi>::new());
        }
        app.add_systems(Update, build_slider.before(UiSystems::Compute));
    }
}
