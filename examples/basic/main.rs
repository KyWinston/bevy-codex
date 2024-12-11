use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use bevy_codex::{prelude::*, resources::CodexSettings};
use bevy_hui::prelude::AutoLoadState;

#[derive(Resource)]
struct LoadTimer(Timer);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "basic ui".into(),
                    name: Some("bevy.app".into()),
                    present_mode: PresentMode::AutoVsync,
                    // Tells Wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
            UiScreensPlugin {
                config: CodexSettings {
                    title: "basic ui".to_string(),
                    ..default()
                },
                game_settings_folder: "game_settings".to_string(),
            },
        ))
        .add_systems(OnEnter(AutoLoadState::Finished), add_camera)
        .add_systems(OnEnter(UiState::Loading), start_load)
        .add_systems(Update, move_to_hud.run_if(in_state(UiState::Loading)))
        .run();
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
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
