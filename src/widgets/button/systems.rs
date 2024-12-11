use std::path::Path;

use bevy::{
    asset::{io::AssetSourceId, AssetPath},
    prelude::*,
};
use bevy_hui::prelude::{HtmlComponents, HtmlNode};

pub fn register_button(assets: Res<AssetServer>, mut html_comps: HtmlComponents) {
    let path = Path::new("bevy-codex").join("navlink.html");
    let source = AssetSourceId::from("embedded");
    let asset_path = AssetPath::from_path(&path).with_source(source);
    html_comps.register("navlink", assets.load(asset_path));
}

pub fn spawn_button(mut commands: Commands, assets: Res<AssetServer>) {
    let path = Path::new("bevy-codex").join("navlink.html");
    let source = AssetSourceId::from("embedded");
    let asset_path = AssetPath::from_path(&path).with_source(source);

    commands.spawn(HtmlNode(assets.load(asset_path)));
}
