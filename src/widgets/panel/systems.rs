use bevy::prelude::*;
use bevy_hui::prelude::HtmlComponents;

pub fn register_panel(assets: Res<AssetServer>, mut comps: HtmlComponents) {
    comps.register(
        "panel",
        assets.load("embedded://bevy_codex/widgets/panel/panel.html"),
    );
}
