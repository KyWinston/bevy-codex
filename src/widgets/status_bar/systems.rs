use bevy::prelude::*;
use bevy_hui::prelude::HtmlComponents;

pub fn register_status_bar(mut comps: HtmlComponents, assets: Res<AssetServer>) {
    comps.register(
        "statusbar",
        assets.load("embedded://bevy_codex/widgets/status_bar/status_bar.html"),
    );
}
