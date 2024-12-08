use bevy::prelude::*;
use bevy_hui::prelude::HtmlNode;

use super::components::Loading;

pub fn load_game_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Loading(Some("...Loading".to_string())),
        HtmlNode(asset_server.load("pages/loading.html")),

    ));
}
