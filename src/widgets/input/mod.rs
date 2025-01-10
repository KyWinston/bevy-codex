use bevy::{asset::embedded_asset, prelude::*};
use bevy_hui_widgets::prelude::HuiInputWidgetPlugin;
use systems::register_input;

pub mod systems;

#[derive(Clone)]
pub struct InputFieldPlugin;

impl Plugin for InputFieldPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "input.html");
        app.add_plugins(HuiInputWidgetPlugin)
            .add_systems(Startup, register_input);
    }
}
