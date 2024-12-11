use bevy::{asset::embedded_asset, prelude::*};
use systems::register_button;

pub mod components;
pub mod systems;

#[derive(Clone)]
pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "navlink.html");
        app.add_systems(PreStartup, register_button);
    }
}
