use bevy::{asset::embedded_asset, prelude::*};
use systems::register_button;

pub mod events;
mod systems;

#[derive(Clone)]
pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "navlink.html");
        embedded_asset!(app, "navlink.css");
        app.add_systems(PreStartup, register_button);
    }
}
