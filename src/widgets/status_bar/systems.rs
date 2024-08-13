use bevy::prelude::*;
use bevy_lunex::prelude::*;

use super::components::StatusBar;

pub fn build_status_bar(
    mut commands: Commands,
    query: Query<(Entity, &StatusBar), Added<StatusBar>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, bar) in &query {
        commands.entity(entity).with_children(|ui| {
            let bar_link = UiLink::<MainUi>::path("Camera/Hud/StatusBar-".to_string() + &bar.label);
            ui.spawn((
                bar_link.clone(),
                UiLayout::boundary()
                    .pos1(Rl(bar.top_left))
                    .pos2(Rl(bar.bottom_right))
                    .pack::<Base>(),
                ImageScaleMode::Sliced(TextureSlicer {
                    border: BorderRect::rectangle(5.0, 5.0),
                    ..default()
                }),
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
