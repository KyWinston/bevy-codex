use bevy::prelude::*;
use bevy_hui::prelude::HtmlNode;

use crate::UiState;

use super::{components::SplashUi, resources::SplashTimer};

pub fn create_splash(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));
    cmd.spawn((SplashUi, HtmlNode(assets.load("pages/splash.html"))));
}

pub fn splash_timer(
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    mut state: ResMut<NextState<UiState>>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        state.set(UiState::MainMenu);
    }
}
