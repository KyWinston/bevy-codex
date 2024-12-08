use bevy::prelude::*;
use bevy_hui::prelude::{HtmlFunctions, HtmlNode};

use crate::{SimulationState, UiState};

pub fn go_to_paused(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(HtmlNode(server.load("pages/pause.html")));
}

pub fn register_pause_actions(mut html_funcs: HtmlFunctions) {
    html_funcs.register(
        "resume_game",
        |In(_): In<Entity>, mut state: ResMut<NextState<SimulationState>>| {
            state.set(SimulationState::Running);
        },
    );

    html_funcs.register(
        "go_to_settings",
        |In(_): In<Entity>, mut state: ResMut<NextState<UiState>>| {
            state.set(UiState::Settings);
        },
    );

    html_funcs.register(
        "go_to_settings",
        |In(_): In<Entity>, mut state: ResMut<NextState<UiState>>| {
            state.set(UiState::Settings);
        },
    );
}
