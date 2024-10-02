use bevy_lunex::{UiDebugPlugin, UiGenericPlugins, UiSystems};
use components::StatusBarUi;
use bevy::prelude::*;
use systems::build_status_bar;
use crate::resources::CodexSettings;


pub mod components;
pub mod systems;

#[derive(Clone)]
pub struct StatusBarPlugin;

impl Plugin for StatusBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiGenericPlugins::<StatusBarUi>::new());
        if app.world().resource::<CodexSettings>().debug {
            app.add_plugins(UiDebugPlugin::<StatusBarUi>::new());
        }
        app.add_systems(Update, build_status_bar.before(UiSystems::Compute));
    }
}
