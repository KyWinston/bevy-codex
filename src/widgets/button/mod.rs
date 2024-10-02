use bevy::prelude::*;
use bevy_lunex::{UiDebugPlugin, UiGenericPlugins, UiSystems};
use components::CustomButtonUi;
use systems::build_button;

use crate::resources::CodexSettings;

pub mod components;
pub mod styles;
pub mod systems;

#[derive(Clone)]
pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiGenericPlugins::<CustomButtonUi>::new(),));
        if app.world().resource::<CodexSettings>().debug {
            app.add_plugins(UiDebugPlugin::<CustomButtonUi>::new());
        }
        app.add_systems(Update, build_button.before(UiSystems::Compute));
    }
}
