use bevy::prelude::*;
use bevy_hui::prelude::HtmlComponents;

pub fn register_input(mut comps: HtmlComponents, assets: Res<AssetServer>) {
    comps.register(
        "input",
        assets.load("embedded://bevy_codex/widgets/input/input.html"),
    );
}
