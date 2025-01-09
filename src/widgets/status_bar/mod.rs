use bevy::{asset::embedded_asset, prelude::*};
use systems::register_status_bar;

pub mod components;
pub mod systems;

#[derive(Clone)]
pub struct StatusBarPlugin;

impl Plugin for StatusBarPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "status_bar.html");
        app.add_systems(Update, register_status_bar);
    }
}
