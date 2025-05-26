use bevy::prelude::*;
use bevy_flair::style::components::NodeStyleSheet;
use bevy_hui::prelude::{HtmlComponents, HtmlFunctions, HtmlNode};

use super::components::Hud;

pub(super) fn open_hud(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((Hud, HtmlNode(server.load("pages/hud.html"))));
}

pub(super) fn register_hud(
    mut comps: HtmlComponents,
    mut funcs: HtmlFunctions,
    assets: Res<AssetServer>,
) {
    comps.register_with_spawn_fn("hud", assets.load("pages/hud.html"), |mut cmd| {
        cmd.insert(Hud);
    });

    funcs.register("append_hud_styles", append_styles);
}

fn append_styles(
    In(entity): In<Entity>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.entity(entity).insert((
        Name::new("hud"),
        NodeStyleSheet::new(asset_server.load("pages/hud.css")),
    ));
}
