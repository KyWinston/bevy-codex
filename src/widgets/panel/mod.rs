use bevy::prelude::*;
use bevy_lunex::{UiDebugPlugin, UiGenericPlugin, UiSystems};
use components::PanelUi;
use systems::build_panel;

use crate::resources::CodexSettings;

pub mod components;
pub mod styles;
pub mod systems;

#[derive(Clone)]
pub struct PanelPlugin;

impl Plugin for PanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiGenericPlugin::<PanelUi>::new(),));
        if app.world().resource::<CodexSettings>().debug {
            app.add_plugins(UiDebugPlugin::<PanelUi>::new());
        }
        app.add_systems(Update, build_panel.before(UiSystems::Compute));
    }
}
