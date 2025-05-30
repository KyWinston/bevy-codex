use bevy::prelude::*;
use bevy_flair::style::components::NodeStyleSheet;
use bevy_hui::prelude::{HtmlComponents, HtmlFunctions};

pub fn register_card(
    mut comps: HtmlComponents,
    mut funcs: HtmlFunctions,
    assets: Res<AssetServer>,
) {
    comps.register(
        "card",
        assets.load("embedded://bevy_codex/widgets/card/card.html"),
    );
    funcs.register("append_card_styles", append_styles);
}

fn append_styles(In(entity): In<Entity>, mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.entity(entity).insert((
        Name::new("card"),
        NodeStyleSheet::new(
            asset_server.load("embedded://bevy_codex/widgets/card/card.css"),
        ),
    ));
}
