use bevy::{asset::embedded_asset, prelude::*};
use systems::register_card;

pub mod systems;

#[derive(Clone)]
pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "card.html");
        embedded_asset!(app, "card.css");

        app.add_systems(PreStartup, register_card);
    }
}
