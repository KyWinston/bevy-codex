use bevy::{color::palettes::css::BLACK, prelude::*};
use bevy_lunex::prelude::*;

use crate::UiState;

use super::{components::Loading, styles::get_loading_text_styles};

pub fn build_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut load_state: ResMut<NextState<UiState>>,
    query: Query<(Entity, &Loading), Added<Loading>>,
) {
    for (route_entity, load_text) in &query {
        load_state.set(UiState::Loading);
        commands.entity(route_entity).with_children(|route| {
            route
                .spawn((
                    UiTreeBundle::<MainUi>::from(UiTree::new2d("Loading")),
                    MovableByCamera,
                ))
                .with_children(|ui| {
                    ui.spawn((
                        UiLink::<MainUi>::path("Load_text"),
                        UiLayout::window()
                            .pos((Rl(80.0) - Ab(60.0), Rl(90.0) - Ab(20.0)))
                            .pack::<Base>(),
                        UiColor::<Base>::new(Color::Srgba(BLACK)),
                        Pickable::IGNORE,
                        UiTextSize::new().size(Rh(10.0)),
                        UiText2dBundle {
                            text: Text::from_section(
                                load_text.0.as_ref().unwrap().to_string(),
                                get_loading_text_styles(&asset_server),
                            ),
                            ..default()
                        },
                    ));
                });
        });
    }
}
