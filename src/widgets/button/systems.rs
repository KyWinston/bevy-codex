use bevy::prelude::*;
use bevy_flair::style::components::NodeStyleSheet;
use bevy_hui::prelude::{HtmlComponents, HtmlFunctions, Tags};

use super::events::BtnClickEvent;

pub fn register_button(
    assets: Res<AssetServer>,
    mut html_comps: HtmlComponents,
    mut html_func: HtmlFunctions,
) {
    html_comps.register(
        "navlink",
        assets.load("embedded://bevy_codex/widgets/button/navlink.html"),
    );

    html_func.register(
        "click",
        |In(entity): In<Entity>, tags: Query<&Tags>, mut clk_ev: EventWriter<BtnClickEvent>| {
            let Some(path) = tags
                .get(entity)
                .ok()
                .and_then(|t| t.get("select_snd").map(|s| s.to_string()))
            else {
                return;
            };
            clk_ev.write(BtnClickEvent(path));
        },
    );
    html_func.register("append_navlink_styles", append_styles);
}

fn append_styles(In(entity): In<Entity>, mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.entity(entity).insert((
        Name::new("navlink"),
        NodeStyleSheet::new(asset_server.load("embedded://bevy_codex/widgets/button/navlink.css")),
    ));
}
