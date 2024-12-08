use bevy::prelude::*;
use bevy_hui::prelude::HtmlNode;

use super::components::Hud;

pub fn open_hud(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((Hud, HtmlNode(server.load("pages/hud.html"))));
}
