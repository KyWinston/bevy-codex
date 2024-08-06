use bevy::{color::palettes::css::BLACK, prelude::*, window::PrimaryWindow};
use bevy_lunex::{prelude::*, Base};

use crate::{
    resources::CodexSettings,
    settings::resources::TomlAsset,
    widgets::{
        panel::components::Panel,
        slider::components::{Knob, Rack},
    },
};

use super::{
    components::Settings,
    resources::{AllSettings, SettingsVal, SettingsVals},
};

pub fn build_settings(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    knobs_q: Query<(Entity, &Knob)>,
    settings: Res<SettingsVals>,
    query: Query<Entity, Added<Settings>>,
    window: Query<&Window, With<PrimaryWindow>>,
    codex_settings: Res<CodexSettings>,
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
                            UiTreeBundle::<MainUi>::from(UiTree::new2d("Settings")),
                            MovableByCamera,
                        ))
                        .with_children(|ui| {
                            let root = UiLink::<MainUi>::path("Root");
                            ui.spawn((
                                root.clone(),
                                UiLayout::window().size(r_size).pack::<Base>(),
                            ));
                            let background = root.add("Background");
                            ui.spawn((
                                background.clone(),
                                UiLayout::window_full().pack::<Base>(),
                                Pickable::IGNORE,
                                UiImage2dBundle::from(asset_server.load("Level_base_diffuse.png")),
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
                                Pickable::IGNORE,
                            ));
                        });
                });
        }
    }
}

pub fn init_settings(mut commands: Commands, settings: Res<SettingsVals>) {
    for (idx, x) in settings.0.iter().enumerate() {
        commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    border: UiRect::all(Val::Px(10.0)),
                    ..default()
                },

                background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                ..default()
            },
            Knob {
                index_tag: idx,
                value: x.value,
            },
        ));
    }
}

pub fn load_settings_toml(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = TomlAsset(asset_server.load("settings.toml"));
    commands.insert_resource(handle);
}

pub fn assign_to_resource(
    mut commands: Commands,
    settings: Res<Assets<AllSettings>>,
    toml: Res<TomlAsset>,
) {
    if let Some(stngs) = settings.get(&toml.0) {
        let mut new_settings: Vec<SettingsVal> = vec![];
        for cat in &stngs.categories {
            for content in cat.contents.as_slice() {
                new_settings.push(content.clone());
            }
        }
        commands.insert_resource(SettingsVals(new_settings.to_vec()));
    }
}
