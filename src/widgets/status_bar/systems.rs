use bevy::prelude::*;
use bevy_lunex::prelude::*;

use super::components::{StatusBar, StatusBarUi};

pub fn build_status_bar(
    mut commands: Commands,
    query: Query<(Entity, &StatusBar), Added<StatusBar>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, bar) in &query {
        commands
            .entity(entity)
            .insert(UiTreeBundle::<StatusBarUi>::from(UiTree::new2d(
                "StatusBar",
            )))
            .with_children(|ui| {
                let bar_link =
                    UiLink::<StatusBarUi>::path("StatusBarFrame-".to_string() + &bar.label);
                ui.spawn((
                    bar_link.clone(),
                    UiLayout::boundary()
                        .pos1(Ab(bar.top_left))
                        .pos2(Ab(bar.bottom_right))
                        .pack::<Base>(),
                    ImageScaleMode::Sliced(TextureSlicer {
                        border: BorderRect::rectangle(5.0, 5.0),
                        ..default()
                    }),
                    UiImage2dBundle {
                        texture: bar
                            .frame
                            .clone()
                            .unwrap_or(asset_server.load("images/ui/barHorizontal_shadow_9.png")),
                        sprite: Sprite { ..default() },
                        ..default()
                    },
                ));
                ui.spawn((
                    bar_link.add("StatusBar"),
                    UiLayout::window().size(Rl((bar.value, 90.0))).pack::<Base>(),
                    UiImage2dBundle {
                        texture: asset_server.load("images/ui/barHorizontal_white_mid.png"),
                        sprite: Sprite {
                            color: Color::Srgba(bar.color.into()),
                            ..default()
                        },
                        ..default()
                    },
                ));
            });
    }
}
