use bevy::prelude::*;
use bevy_hui::prelude::{HtmlFunctions, HtmlNode, TemplateProperties};
use bevy_hui_widgets::prelude::{Slider, SliderChangedEvent};

use super::{
    components::SettingsPg,
    resources::{GameSettings, SettingsHandle},
};
use crate::UiState;

pub fn go_to_settings(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((SettingsPg, HtmlNode(server.load("pages/settings.html"))));
}

pub fn register_settings_actions(
    mut html_funcs: HtmlFunctions,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    html_funcs.register("settings", init_settings);
    html_funcs.register(
        "back",
        |In(_): In<Entity>, mut state: ResMut<NextState<UiState>>| {
            state.set(UiState::MainMenu);
        },
    );
    html_funcs.register("apply", apply_all_settings);
    let settings = SettingsHandle(asset_server.load("game.settings.ron"));
    commands.insert_resource(settings);
}

fn init_settings(
    In(entity): In<Entity>,
    setting_handle: Res<SettingsHandle>,
    settings: Res<Assets<GameSettings>>,
    mut cmd: Commands,
    // mut sliders: Query<&mut Slider>,
    server: Res<AssetServer>,
) {
    if let Some(setting) = settings.get(setting_handle.0.id()) {
        cmd.entity(entity).with_children(|cmd| {
            for (key, _value) in setting.audio_settings.iter() {
                println!("{:?}", key);
                let _slider_ent = cmd
                    .spawn((
                        HtmlNode(server.load("embedded://bevy_codex/widgets/slider/slider.html")),
                        TemplateProperties::default().with("label", key),
                    ))
                    .id();
                // if let Ok(slider) = sliders.get_mut(slider_ent) {
                //     slider. = value;
                // }
            }
        });
    }
}

pub fn update_settings(
    mut slider_ev: EventReader<SliderChangedEvent>,
    setting_handle: Res<SettingsHandle>,
    slider: Query<(Entity, &Slider, &TemplateProperties)>,
    mut settings: ResMut<Assets<GameSettings>>,
) {
    for ev in slider_ev.read() {
        let slider_val = ev.value;
        if let Some(setting) = settings.get_mut(setting_handle.0.id()) {
            if let Ok(label) = &slider.get(ev.slider) {
                if let Some(audio_setting) =
                    setting.audio_settings.get_mut(&label.2["label"].clone())
                {
                    *audio_setting = slider_val;
                }
            }
        }
    }
}

pub fn apply_all_settings(In(_): In<Entity>) {}
