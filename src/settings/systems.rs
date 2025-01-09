use bevy::prelude::*;
use bevy_hui::prelude::{HtmlFunctions, HtmlNode};

use crate::UiState;
use super::components::SettingsPg;

pub fn go_to_settings(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((SettingsPg, HtmlNode(server.load("pages/settings.html"))));
}

pub fn register_settings_actions(mut html_funcs: HtmlFunctions) {
    html_funcs.register(
        "back",
        |In(_): In<Entity>, mut state: ResMut<NextState<UiState>>| {
            state.set(UiState::MainMenu);
        },
    );
}
