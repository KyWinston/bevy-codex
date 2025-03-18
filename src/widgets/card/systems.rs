use bevy::prelude::*;
use bevy_hui::prelude::HtmlComponents;

pub fn register_card(mut comps: HtmlComponents, assets: Res<AssetServer>) {
    comps.register(
        "card",
        assets.load("embedded://bevy_codex/widgets/card/card.html"),
    );
}

// fn init_inventory(In(entity): In<Entity>, mut cmd: Commands, server: Res<AssetServer>) {
//     cmd.entity(entity).with_children(|cmd| {
//         for i in 0..200 {
//             cmd.spawn((
//                 HtmlNode(server.load("demo/card.html")),
//                 TemplateProperties::default()
//                     .with("title", &format!("item {i}"))
//                     .with("bordercolor", if i % 2 == 0 { "#FFF" } else { "#F88" }),
//             ));
//         }
//     });
// }
