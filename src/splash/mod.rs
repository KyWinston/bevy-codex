use bevy::prelude::*;

use events::SplashScreenSkipEvent;
use systems::{create_splash, splash_timer};

use crate::{main_menu::components::MainMenu, resources::CodexSettings, UiState};

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
            .add_systems(Update, splash_timer)
            .add_systems(OnEnter(UiState::MainMenu), |mut commands: Commands| {
                commands.spawn(MainMenu);
            });
    }
}
