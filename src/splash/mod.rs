use bevy::prelude::*;

use components::SplashUi;
use events::SplashScreenSkipEvent;
use systems::{create_splash, splash_timer};

use crate::{resources::CodexSettings, UiState};

pub(crate) mod components;
pub(crate) mod events;
pub(crate) mod resources;
pub(crate) mod systems;

pub(crate) struct SplashReelPlugin;
impl Plugin for SplashReelPlugin {
    fn build(&self, app: &mut App) {
        let _codex_settings = app.world().resource::<CodexSettings>();
        app.add_event::<SplashScreenSkipEvent>()
            .add_systems(Startup, create_splash)
            .add_systems(Update, splash_timer.run_if(in_state(UiState::Splash)))
            .add_systems(
                OnExit(UiState::Splash),
                |mut commands: Commands, node: Query<Entity, With<SplashUi>>| {
                    if let Ok(node) = node.single() {
                        commands.entity(node).despawn();
                    }
                },
            );
    }
}
