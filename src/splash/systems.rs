use bevy::{prelude::*, window::PrimaryWindow};
use bevy_lunex::{
    prelude::{MainUi, Rl, UiNodeTreeInitTrait, UiTree},
    Base, MovableByCamera, PackageLayout, UiImage2dBundle, UiLayout, UiLink, UiTreeBundle,
};

use super::components::{SplashScreen, SplashTimer};
use crate::{main_menu::components::MainMenu, UiState};

pub fn build_splash(
    mut commands: Commands,
    assets: Res<AssetServer>,
    window: Query<&Window, With<PrimaryWindow>>,
    query: Query<Entity, Added<SplashScreen>>,
) {
    for route_entity in &query {
        if let Ok(resolution) = window.get_single() {
            let r_size = (resolution.width(), resolution.height());
            commands
                .entity(route_entity)
                .insert(SpatialBundle::default())
                .with_children(|route| {
                    route
                        .spawn((
                            UiTreeBundle::<MainUi>::from(UiTree::new2d("MainMenu")),
                            MovableByCamera,
                        ))
                        .with_children(|ui| {
                            let root = UiLink::<MainUi>::path("Root");
                            ui.spawn((
                                root.clone(),
                                UiLayout::window().size(r_size).pack::<Base>(),
                            ));
                            // Spawn the background
                            ui.spawn((
                                UiLink::<MainUi>::path("Background"), // You can see here that we used existing "root" link to create chained link (same as "Root/Background")
                                UiLayout::boundary()
                                    .pos1(Rl(40.0))
                                    .pos2(Rl(60.0))
                                    .pack::<Base>(),
                                UiImage2dBundle::from(assets.load("branding/icon.png")), // We use this bundle to add background image to our node
                            ));
                        });
                });
        }
    }
}
pub fn count_down(
    mut commands: Commands,
    mut state: ResMut<NextState<UiState>>,
    mut timer: ResMut<SplashTimer>,
    splash: Query<Entity, With<SplashScreen>>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.finished() {
        state.set(UiState::MainMenu);
        for route_entity in &splash {
            println!("despawn splash");
            commands.entity(route_entity).despawn_recursive();
        }
        commands.spawn(MainMenu);
    }
}
