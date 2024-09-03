use bevy::prelude::*;
use bevy_lunex::UiSystems;
use components::SplashTimer;
use systems::build_splash;

use crate::UiState;

use self::systems::count_down;

pub mod components;
pub mod systems;

pub struct SplashPlugin;
impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<SplashTimer>(SplashTimer(Timer::from_seconds(5.0, TimerMode::Once)))
            .add_systems(
                Update,
                (
                    build_splash.before(UiSystems::Compute),
                    count_down.run_if(in_state(UiState::Splash)),
                ),
            );
    }
}
