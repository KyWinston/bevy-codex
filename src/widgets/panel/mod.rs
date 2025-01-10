use bevy::{asset::embedded_asset, prelude::*};
use systems::register_panel;

pub mod systems;

#[derive(Clone)]
pub struct PanelPlugin;

impl Plugin for PanelPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "panel.html");
        app.add_systems(Startup, register_panel);
    }
}
