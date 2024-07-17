use bevy::prelude::*;
use bevy::{
    asset::AssetServer,
    color::{palettes::css::BLACK, Color},
    text::TextStyle,
};

use crate::resources::CodexSettings;

pub fn get_button_text_styles(
    asset_server: &Res<AssetServer>,
    codex_settings: &Res<CodexSettings>,
) -> TextStyle {
    TextStyle {
        font: asset_server.load(codex_settings.font.clone()),
        font_size: 80.0,
        color: Color::srgb_from_array(BLACK.to_f32_array_no_alpha()),
    }
}
