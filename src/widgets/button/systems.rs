use bevy::prelude::*;
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
    )
}
