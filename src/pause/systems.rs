use bevy::color::palettes::css::BLACK;
use bevy::prelude::EventReader;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_lunex::prelude::*;

use crate::widgets::button::components::{CustomButton, CustomButtonRef};
use crate::widgets::panel::components::Panel;
use crate::UiState;

use super::components::{Pause, PauseButton};

pub fn build_pause(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    query: Query<Entity, Added<Pause>>,
) {
    for route_entity in &query {
        if let Ok(resolution) = window.get_single() {
            let r_size = (resolution.width(), resolution.height());
            commands
                .entity(route_entity)
                .insert(SpatialBundle::default())
                .with_children(|route| {
                    route
                        .spawn((
                            UiTreeBundle::<MainUi>::from(UiTree::new2d("PauseMenu")),
                            MovableByCamera,
                        ))
                        .with_children(|ui| {
                            let root = UiLink::<MainUi>::path("Root");
                            ui.spawn((
                                root.clone(),
                                UiLayout::window().size(r_size).pack::<Base>(),
                            ));
                            let background = UiLink::<MainUi>::path("Background");

                            ui.spawn((UiLayout::window_full().pack::<Base>(), Pickable::IGNORE));

                            let panel: UiLink = background.add("panel");

                            let buttons = [
                                CustomButtonRef {
                                    link: PauseButton::Resume.str(),
                                },
                                CustomButtonRef {
                                    link: PauseButton::Settings.str(),
                                },
                                CustomButtonRef {
                                    link: PauseButton::Menu.str(),
                                },
                            ];
                            ui.spawn((
                                panel.clone(),
                                UiLayout::window()
                                    .size(Rl((40.0, 80.0)))
                                    .pos(Rl(10.0))
                                    .pack::<Base>(),
                                Panel {
                                    text: Some("Paused".to_string()),
                                    color: BLACK.into(),
                                    content: buttons.to_vec(),
                                    ..default()
                                },
                            ));
                        });
                });
        }
    }
}

/// In this system we run our button click logic
pub fn pause_button_clicked_system(
    mut commands: Commands,
    mut events: EventReader<UiClickEvent>,
    mut ui_state: ResMut<NextState<UiState>>,
    query: Query<&CustomButton>,
    menu_q: Query<Entity, With<Pause>>,
) {
    for event in events.read() {
        if let Ok(ent) = menu_q.get_single() {
            if let Ok(button) = query.get(event.target) {
                if button.text == PauseButton::Resume.str() {
                    commands.entity(ent).despawn_recursive();
                } else if button.text == PauseButton::Menu.str() {
                    ui_state.set(UiState::MainMenu);
                }
            }
        }
    }
}

pub fn despawn_pause(mut commands: Commands, query: Query<Entity, Added<Pause>>) {
    for ent in &query {
        commands.entity(ent).despawn_recursive();
    }
}
