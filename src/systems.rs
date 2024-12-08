use bevy::prelude::*;

use crate::{components::Quit, UiState};

pub fn init_ui_cam(mut commands: Commands, mut state: ResMut<NextState<UiState>>) {
    commands.spawn(Camera2d);
    println!("go to menu");
    state.set(UiState::MainMenu);
}
pub fn exit(mut app_exit_event_writer: EventWriter<AppExit>, quit: Query<Entity, Added<Quit>>) {
    for _ in &quit {
        app_exit_event_writer.send(AppExit::Success);
    }
}


// /// Function to return camera will all appropriate settings
// pub fn camera() -> (MainUi, Camera, Transform, InheritedVisibility) {
//     (
//         MainUi,
//         Camera::default(),
//         Transform::from_xyz(0.0, 0.0, 1000.0),
//         InheritedVisibility::default(),
//     )
// }

// pub fn create_root(commands: &mut Commands, r_size: (f32, f32)) {
//     commands
//         .spawn((Transform::default(), Visibility::Inherited))
//         .with_children(|route| {
//             route
//                 .spawn((
//                     UiTreeBundle::<MainUi>::from(UiTree::new2d("Main")),
//                     SourceFromCamera,
//                 ))
//                 .with_children(|ui| {
//                     let root = UiLink::<MainUi>::path("Root");
//                     ui.spawn((root.clone(), UiLayout::window().size(r_size).pack::<Base>()));
//                 });
//         });
// }
// pub fn button_click(mut events: EventReader<UiClickEvent>, mut ev_w: EventWriter<SelectEvent>) {
//     for event in events.read() {
//         ev_w.send(SelectEvent(event.clone()));
//     }
// }
