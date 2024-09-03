use bevy::prelude::*;
use bevy_lunex::prelude::*;

use crate::widgets::{
    button::components::CustomButtonRef, list::components::List, status_bar::components::StatusBar,
};

use super::{
    components::{Panel, PanelUi},
    styles::get_panel_text_styles,
};

pub fn build_button_panel(
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<(Entity, &Panel<CustomButtonRef>), Added<Panel<CustomButtonRef>>>,
) {
    for (entity, panel) in &query {
        let panel_ent = build_panel::<CustomButtonRef>(&mut commands, &assets, entity, panel);
        commands.entity(panel_ent).with_children(|ui| {
            let mut buttons = vec![];
            for btn in &panel.content {
                buttons.push(btn.link.to_string())
            }
            ui.spawn((
                UiLink::<PanelUi>::path("Panel/List"),
                UiLayout::window()
                    .size(Rl((80.0, 87.0)))
                    .pos(Rl((10.0, 13.0)))
                    .pack::<Base>(),
                List {
                    items: buttons.to_vec(),
                    ..default()
                },
            ));
        });
    }
}

pub fn build_status_bar_panel(
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<(Entity, &Panel<StatusBar>), Added<Panel<StatusBar>>>,
) {
    for (entity, panel) in &query {
        let panel_ent = build_panel::<StatusBar>(&mut commands, &assets, entity, panel);
        commands.entity(panel_ent).with_children(|ui| {
            for bar in panel.content.clone() {
                ui.spawn((
                    UiLink::<PanelUi>::path("Panel/Statusbar-".to_string() + &bar.label),
                    UiLayout::solid().size((1920.0,1080.0)).scaling(Scaling::Fit).pack::<Base>(),
                    bar,
                ));
            }
        });
    }
}

fn build_panel<T>(
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    entity: Entity,
    panel: &Panel<T>,
) -> Entity {
    commands
        .entity(entity)
        .insert((UiTreeBundle::<PanelUi>::from(UiTree::new2d("Panel")),))
        .with_children(|ui| {
            let panel_link = UiLink::<PanelUi>::path("Panel");
            let mut panel_bundle = ui.spawn((
                panel_link.clone(),
                UiLayout::window_full().pack::<Base>(),
                Pickable::IGNORE,
                UiColor::<Base>::new(panel.color),
            ));
            if panel.texture.is_some() {
                panel_bundle.insert((
                    ImageScaleMode::Sliced(TextureSlicer {
                        border: BorderRect::rectangle(10.0, 10.0),
                        ..default()
                    }),
                    UiImage2dBundle {
                        texture: panel.texture.as_ref().unwrap().clone(),
                        sprite: Sprite {
                            color: Color::Srgba(panel.color.into()),
                            ..default()
                        },
                        ..default()
                    },
                ));
            }
            if panel.text.is_some() {
                ui.spawn((
                    panel_link.add("Heading"),
                    UiLayout::window().pack::<Base>(),
                    UiTextSize::new().size(Rh(13.0)),
                    UiText2dBundle {
                        text: Text::from_section(
                            panel.text.clone().unwrap(),
                            get_panel_text_styles(assets),
                        ),
                        ..default()
                    },
                    Pickable::IGNORE,
                ));
            }
        })
        .id()
}
