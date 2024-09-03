use bevy::{prelude::*, window::PrimaryWindow};
use bevy_lunex::{prelude::*, Base};


use super::components::{SettingsPg, SettingsPgUi};

pub fn build_settings(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Query<&Window, With<PrimaryWindow>>,

    query: Query<Entity, Added<SettingsPg>>,
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
                            let background = UiLink::<SettingsPgUi>::path("Background");
                            ui.spawn((
                                background.clone(),
                                UiLayout::window_full().pack::<Base>(),
                                Pickable::IGNORE,
                                UiImage2dBundle::from(asset_server.load("Level_base_diffuse.png")),
                            ));
                        });
                });
        }
    }
}
