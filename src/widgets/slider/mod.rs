use bevy::{asset::embedded_asset, prelude::*};
use bevy_hui_widgets::prelude::HuiSliderWidgetPlugin;
use systems::{register_slider, update_slider_target_text};

pub mod systems;

#[derive(Clone)]
pub struct SliderPlugin;

impl Plugin for SliderPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "slider.html");
        app.add_plugins(HuiSliderWidgetPlugin)
            .add_systems(Update, update_slider_target_text)
            .add_systems(Startup, register_slider);
    }
}
