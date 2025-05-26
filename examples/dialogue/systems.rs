use bevy::prelude::*;
use bevy_codex::{prelude::UiState, widgets::dialogue_view::events::RunDialogueEvent};

use crate::resources::LoadTimer;

pub fn run_dialog(key: Res<ButtonInput<KeyCode>>, mut run_ev: EventWriter<RunDialogueEvent>) {
    if key.just_released(KeyCode::KeyP) {
        run_ev.write(RunDialogueEvent("Hello".to_string()));
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}

pub fn start_load(mut commands: Commands) {
    commands.insert_resource(LoadTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

pub fn move_to_hud(
    mut next_state: ResMut<NextState<UiState>>,
    mut timer: ResMut<LoadTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        next_state.set(UiState::Hud);
    }
}
