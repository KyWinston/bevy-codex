use bevy::prelude::*;
use bevy_flair::style::components::NodeStyleSheet;
use bevy_hui::prelude::{HtmlComponents, HtmlFunctions};

pub fn register_status_bar(
    mut comps: HtmlComponents,
    mut funcs: HtmlFunctions,
    assets: Res<AssetServer>,
) {
    comps.register(
        "statusbar",
        assets.load("embedded://bevy_codex/widgets/status_bar/status_bar.html"),
    );
    funcs.register("append_status_bar_styles", append_styles);
}

fn append_styles(In(entity): In<Entity>, mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.entity(entity).insert((
        Name::new("status_bar"),
        NodeStyleSheet::new(
            asset_server.load("embedded://bevy_codex/widgets/status_bar/status_bar.css"),
        ),
    ));
}
