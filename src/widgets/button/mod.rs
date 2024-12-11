use bevy::{asset::embedded_asset, prelude::*};
use bevy_hui::prelude::HtmlTemplate;
use systems::{register_button, spawn_button};

pub const BUTTON_HANDLE: Handle<HtmlTemplate> = Handle::weak_from_u128(17083338353718453747);

pub mod components;
pub mod systems;

#[derive(Clone)]
pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(
            app,
            "assets/widgets",
            "../../../assets/widgets/navlink.html"
        );
        app.add_systems(PreStartup, register_button)
            .add_systems(Startup, spawn_button);
    }
}
