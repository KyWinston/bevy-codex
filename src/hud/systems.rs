use bevy::prelude::*;
use bevy_hui::prelude::{HtmlComponents, HtmlNode};

use super::components::Hud;

pub fn open_hud(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((Hud, HtmlNode(server.load("pages/hud.html"))));
}

pub fn register_hud(mut comps: HtmlComponents, assets: Res<AssetServer>) {
    comps.register_with_spawn_fn("hud", assets.load("pages/hud.html"), |mut cmd| {
        cmd.insert(Hud);
    });
}
