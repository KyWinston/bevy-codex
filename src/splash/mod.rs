use bevy::prelude::*;

use components::SplashUi;
use events::SplashScreenSkipEvent;
use systems::{create_splash, splash_timer};

use crate::{resources::CodexSettings, UiState};

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

pub struct SplashReelPlugin;
impl Plugin for SplashReelPlugin {
    fn build(&self, app: &mut App) {
        let _codex_settings = app.world().resource::<CodexSettings>();
        app.add_event::<SplashScreenSkipEvent>()
            .add_systems(Startup, create_splash)
            .add_systems(Update, splash_timer.run_if(in_state(UiState::Splash)))
            .add_systems(
                OnExit(UiState::Splash),
                |mut commands: Commands, node: Query<Entity, With<SplashUi>>| {
                    if let Ok(node) = node.get_single() {
                        commands.entity(node).despawn_recursive();
                    }
                },
            );
    }
}
