use bevy::{color::palettes::css::BLACK, prelude::*, window::PrimaryWindow};
use bevy_lunex::{prelude::*, Base};

use crate::widgets::{
    panel::components::Panel,
    slider::components::{Knob, Rack},
};

use super::{
    components::{SettingsPanel, SettingsPg, SettingsPgUi},
    resources::Settings,
};

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
                                    UiImage2dBundle::from(
                                        asset_server.load("Level_base_diffuse.png"),
                                    ),
                                ));
                                ui.spawn((
                                    background.add("Panel"),
                                    UiLayout::window()
                                        .size(Rl((40.0, 80.0)))
                                        .pos(Rl((10.0, 10.0)))
                                        .pack::<Base>(),
                                    Panel {
                                        text: Some("Settings".to_string()),
                                        color: BLACK.into(),
                                        ..default()
                                    },
                                    SettingsPanel,
                                    Pickable::IGNORE,
                                ));
                            });
                    });
            }
        }
    }


pub fn init_settings(
    mut commands: Commands,
    panel: Query<Entity, Added<SettingsPanel>>,
    settings: Res<Settings>,
) {
    let mut knobs = vec![];
    for p in &panel {
        for (label, value) in settings.sound_settings.iter() {
            knobs.push(
                commands
                    .spawn(Knob {
                        index_tag: label.to_string(),
                        value: *value,
                    })
                    .id(),
            );
        }
        commands.entity(p).insert_children(0, &knobs);
    }
}
