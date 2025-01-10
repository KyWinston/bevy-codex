use bevy::prelude::*;
use bevy_hui::prelude::{HtmlFunctions, HtmlNode, TemplateProperties};

use crate::{components::Quit, resources::CodexSettings, UiState};

use super::components::MainMenu;

pub fn go_to_main(mut commands: Commands, server: Res<AssetServer>, settings: Res<CodexSettings>) {
    commands.spawn((
        MainMenu,
        HtmlNode(server.load("pages/menu.html")),
        TemplateProperties::default().with("title", &settings.title),
    ));
}

pub fn register_menu_actions(mut html_funcs: HtmlFunctions) {
    html_funcs.register(
        "start_game",
        |In(_): In<Entity>, mut state: ResMut<NextState<UiState>>| {
            state.set(UiState::Loading);
        },
    );

    html_funcs.register(
        "go_to_settings",
        |In(_): In<Entity>, mut state: ResMut<NextState<UiState>>| {
            state.set(UiState::Settings);
        },
    );

    html_funcs.register("quit_game", |In(_): In<Entity>, mut commands: Commands| {
        commands.spawn(Quit);
    });

}
