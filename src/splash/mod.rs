use std::time::Duration;

use bevy::{
    color::palettes::css::{NAVY, WHITE},
    prelude::*,
};
use bevy_splash_screen::{SplashAssetType, SplashItem, SplashPlugin, SplashScreen, WaitScreenType};
use bevy_tweening::EaseFunction;

use crate::{main_menu::components::MainMenu, resources::CodexSettings, UiState};
pub mod components;

pub struct SplashReelPlugin;
impl Plugin for SplashReelPlugin {
    fn build(&self, app: &mut App) {
        let codex_settings = app.world().resource::<CodexSettings>();
        let mut pages: Vec<SplashItem> = vec![];
        for page in &codex_settings.splash_pages {
            pages.push(SplashItem {
                asset: SplashAssetType::SingleText(
                    Text::from_sections([TextSection::new(
                        page.splash_header.clone(),
                        TextStyle {
                            font_size: 32.,
                            color: WHITE.into(),
                            ..default()
                        },
                    )])
                    .with_justify(JustifyText::Center),
                    codex_settings.font.to_string(),
                ),
                tint: WHITE.into(),
                width: Val::Percent(30.),
                height: Val::Px(100.),
                ease_function: EaseFunction::QuarticInOut.into(),
                duration: Duration::from_secs_f32(page.splash_duration),
                is_static: false,
            });
            pages.push(SplashItem {
                asset: SplashAssetType::SingleImage(page.splash_logo.to_string() + ".png"),
                tint: WHITE.into(),
                width: Val::Percent(25.),
                height: Val::Auto,
                ease_function: EaseFunction::QuinticInOut.into(),
                duration: Duration::from_secs_f32(page.splash_duration),
                is_static: false,
            });
            pages.push(SplashItem {
                asset: SplashAssetType::SingleText(
                    Text::from_sections([TextSection::new(
                        page.splash_footer.clone(),
                        TextStyle {
                            font_size: 32.,
                            color: WHITE.into(),
                            ..default()
                        },
                    )])
                    .with_justify(JustifyText::Center),
                    codex_settings.font.to_string(),
                ),
                tint: WHITE.into(),
                width: Val::Percent(30.),
                height: Val::Px(120.),
                ease_function: EaseFunction::QuinticInOut.into(),
                duration: Duration::from_secs_f32(page.splash_duration),
                is_static: false,
            });
        }
        app.add_plugins(
            SplashPlugin::new(UiState::Splash, UiState::MainMenu)
                .skipable()
                .add_screen(SplashScreen {
                    brands: pages,
                    background_color: NAVY.into(),
                    wait_to_start: WaitScreenType::AfterEnd,
                    ..default()
                }),
        )
        .add_systems(OnEnter(UiState::MainMenu), |mut commands: Commands| {
            commands.spawn(MainMenu);
        });
    }
}
