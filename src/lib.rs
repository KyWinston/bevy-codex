//! The purpose of bevy-codex, is to be able to quickly customize essential ui pages for the navigation
//! between the main menu, paue menu, loading state, and other crucial ui layouts
//! this crate uses bevy-hui as a means to allow you to use html to customize the pages
//! https://github.com/Lommix/bevy_hui

use bevy::{prelude::*, window::SystemCursorIcon};
use bevy_hui::HuiPlugin;
use bevy_yarnspinner::prelude::YarnSpinnerPlugin;
use hud::HudPlugin;
use loading::LoadingPlugin;
use main_menu::MainMenuPlugin;
use pause::PausePlugin;
use prelude::*;
use resources::{CodexSettings, CursorIcons};

use splash::SplashReelPlugin;
use systems::exit;
use widgets::WidgetPlugins;

pub mod prelude {
    use crate::resources::CodexSettings;
    use bevy::{prelude::Component, reflect::Reflect, state::state::States};

    ///The different sub states for the game when you are on the hud
    #[derive(Default, States, Debug, Reflect, Hash, Eq, PartialEq, Clone)]
    pub enum SimulationState {
        #[default]
        Running,
        Paused,
        Editor,
    }
    ///Top level states. the splash state runs for an amount of time configured in the plugin, before going to the main menu
    #[derive(Default, States, Component, Reflect, Debug, Hash, Eq, PartialEq, Clone)]
    pub enum UiState {
        MainMenu,
        Loading,
        Settings,
        Hud,
        #[default]
        Splash,
        Debug,
    }

    #[derive(Clone)]
    pub struct UiScreensPlugin {
        pub game_settings_folder: String,
        pub config: CodexSettings,
    }
}

pub mod components;
pub mod hud;
pub mod loading;
pub mod main_menu;
pub mod pause;
pub mod resources;
pub mod splash;
pub mod systems;
pub mod widgets;

impl Plugin for UiScreensPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<CodexSettings>(self.config.clone())
            .add_plugins((
                YarnSpinnerPlugin::default(),
                MainMenuPlugin,
                PausePlugin,
                WidgetPlugins,
                SplashReelPlugin,
                HudPlugin,
                HuiPlugin,
                LoadingPlugin,
            ))
            .init_state::<SimulationState>()
            .init_state::<UiState>()
            .insert_resource(CursorIcons(vec![SystemCursorIcon::Default]))
            .add_systems(Update, exit);
    }
}
