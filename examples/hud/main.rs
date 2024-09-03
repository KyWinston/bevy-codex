use bevy::{
    color::palettes::css::{BLUE, RED, WHITE},
    prelude::*,
};
use bevy_codex::{
    hud::components::{Hud, SurfaceHud},
    prelude::*,
    resources::CodexSettings,
    widgets::{panel::components::Panel, status_bar::components::StatusBar},
};

#[derive(Resource)]
struct LoadTimer(Timer);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            UiScreensPlugin {
                settings: CodexSettings {
                    title: "Hud Example".to_string(),
                    button_color: WHITE.into(),
                    debug: true,
                    ..default()
                },
            },
        ))
        .add_systems(OnEnter(UiState::Loading), start_load)
        .add_systems(
            Update,
            (show_hud, move_to_hud.run_if(in_state(UiState::Loading))),
        )
        .run();
}

fn start_load(mut commands: Commands) {
    commands.insert_resource(LoadTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

fn move_to_hud(
    mut commands: Commands,
    mut next_state: ResMut<NextState<UiState>>,
    mut timer: ResMut<LoadTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        next_state.set(UiState::Hud);
        commands.spawn(Hud);
    }
}

fn show_hud(hud_q: Query<Entity, Added<SurfaceHud>>, mut commands: Commands) {
    for q in &hud_q {
        let mut bars = vec![];
        let mut offset = 10.0;
        for (label, color) in [("Health".into(), RED.into()), ("Mana".into(), BLUE.into())] {
            bars.push(StatusBar {
                label,
                color,
                top_left: (10.0, 0.0 + offset),
                bottom_right: (100.0, 30.0 + offset),
                ..default()
            });
            offset += 40.0;
        }
        commands.entity(q).insert(Panel {
            content: bars,
            ..default()
        });
    }
}
