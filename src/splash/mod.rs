use std::time::Duration;

use bevy::{
    color::palettes::css::{NAVY, WHITE},
    prelude::*,
};
use bevy_splash_screen::{SplashAssetType, SplashItem, SplashPlugin, SplashScreen};
use bevy_tweening::EaseFunction;

use crate::{main_menu::components::MainMenu, UiState};
pub mod components;

pub struct SplashReelPlugin;
impl Plugin for SplashReelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            SplashPlugin::new(UiState::Splash, UiState::MainMenu)
                .skipable()
                .add_screen(SplashScreen {
                    brands: vec![
                        SplashItem {
                            asset: SplashAssetType::SingleText(
                                Text::from_sections([TextSection::new(
                                    "Geksupport & JazPixel Present",
                                    TextStyle {
                                        font_size: 32.,
                                        color: WHITE.into(),
                                        ..default()
                                    },
                                )])
                                .with_justify(JustifyText::Center),
                                "fonts/FiraSans-Bold.ttf".to_string(),
                            ),
                            tint: WHITE.into(),
                            width: Val::Percent(30.),
                            height: Val::Px(100.),
                            ease_function: EaseFunction::QuarticInOut.into(),
                            duration: Duration::from_secs_f32(5.),
                            is_static: false,
                        },
                        SplashItem {
                            asset: SplashAssetType::SingleImage("branding/icon.png".to_string()),
                            tint: WHITE.into(),
                            width: Val::Percent(25.),
                            height: Val::Auto,
                            ease_function: EaseFunction::QuinticInOut.into(),
                            duration: Duration::from_secs_f32(5.),
                            is_static: true,
                        },
                        SplashItem {
                            asset: SplashAssetType::SingleText(
                                Text::from_sections([TextSection::new(
                                    "A Game Powered by BevyEngine",
                                    TextStyle {
                                        font_size: 32.,
                                        color: WHITE.into(),
                                        ..default()
                                    },
                                )])
                                .with_justify(JustifyText::Center),
                                "fonts/FiraSans-Bold.ttf".to_string(),
                            ),
                            tint: WHITE.into(),
                            width: Val::Percent(30.),
                            height: Val::Px(120.),
                            ease_function: EaseFunction::QuinticInOut.into(),
                            duration: Duration::from_secs_f32(5.),
                            is_static: false,
                        },
                    ],
                    background_color: BackgroundColor(NAVY.into()),
                    ..default()
                }),
        )
        .add_systems(OnEnter(UiState::MainMenu), |mut commands: Commands| {
            commands.spawn(MainMenu);
        });
    }
}
