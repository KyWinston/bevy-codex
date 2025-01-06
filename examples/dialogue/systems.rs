use bevy::prelude::*;
use bevy_codex::prelude::UiState;
use bevy_yarnspinner::prelude::YarnProject;

use crate::resources::LoadTimer;

pub fn run_dialog(
    mut commands: Commands,
    project: Res<YarnProject>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if key.just_released(KeyCode::KeyP) {
        let mut dialogue_runner = project.create_dialogue_runner();
        dialogue_runner.start_node("Hello");
        println!("");
        commands.spawn(dialogue_runner);
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
