use bevy::prelude::*;
use bevy_codex::{prelude::*, resources::CodexSettings};

#[derive(Resource)]
struct LoadTimer(Timer);
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            UiScreensPlugin {
                config: CodexSettings::default(),
                game_settings_folder: "game_settings".to_string(),
            },
        ))
        .add_systems(OnEnter(UiState::Loading), start_load)
        .add_systems(Update, move_to_hud.run_if(in_state(UiState::Loading)))
        .run();
}

fn start_load(mut commands: Commands) {
    commands.insert_resource(LoadTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

fn move_to_hud(
    mut next_state: ResMut<NextState<UiState>>,
    mut timer: ResMut<LoadTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        next_state.set(UiState::Hud);
    }
}
