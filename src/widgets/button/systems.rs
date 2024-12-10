use bevy::prelude::*;
use bevy_hui::prelude::HtmlNode;

pub fn register_button(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(HtmlNode(assets.load("embedded://bevy_codex/widgets/navlink.html")));
}
