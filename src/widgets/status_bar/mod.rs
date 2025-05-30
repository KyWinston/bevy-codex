use bevy::{asset::embedded_asset, prelude::*};
use systems::register_status_bar;

pub mod systems;

#[derive(Clone)]
pub struct StatusBarPlugin;

impl Plugin for StatusBarPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "status_bar.html");
        embedded_asset!(app, "status_bar.css");

        app.add_systems(Startup, register_status_bar);
    }
}
