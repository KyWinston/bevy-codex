use bevy::prelude::*;
use bevy_hui::prelude::HtmlTemplate;
use systems::register_button;

pub const BUTTON_HANDLE: Handle<HtmlTemplate> = Handle::weak_from_u128(17083338353718453747);

pub mod components;
pub mod systems;

#[derive(Clone)]
pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, register_button);
    }
}
