use bevy::prelude::*;
use bevy_hui::prelude::HtmlComponents;

pub fn register_button(assets: Res<AssetServer>, mut html_comps: HtmlComponents) {
    html_comps.register(
        "navlink",
        assets.load("embedded://bevy_codex/widgets/button/navlink.html"),
    );
}
