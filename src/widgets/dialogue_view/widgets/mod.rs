use bevy::asset::{load_internal_binary_asset, weak_handle};
use bevy::prelude::*;
use systems::{load_font, load_image};

pub const MEDIUM: Handle<Font> = weak_handle!("dd7e6837-6eaa-4c92-855d-510db2334128");
pub const CONTINUE_INDICATOR: Handle<Image> = weak_handle!("1e43baa1-645f-41ad-97f3-4e79170b8266");

pub mod systems;

pub fn ui_assets_plugin(app: &mut App) {
    load_internal_binary_asset!(
        app,
        MEDIUM,
        "../../../../assets/fonts/FiraSans-Bold.ttf",
        load_font
    );

    load_internal_binary_asset!(
        app,
        CONTINUE_INDICATOR,
        "../../../../assets/images/ui/arrow_basic_s.png",
        load_image
    );
}
