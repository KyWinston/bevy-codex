use bevy::{color::palettes::css::YELLOW, prelude::*, sprite::Anchor};
use bevy_lunex::prelude::*;

use crate::{components::Quit, events::SelectEvent, splash::components::SplashScreen};

pub fn init_ui_cam(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(camera()).with_children(|cam| {
        let mut cursor = Cursor2d::new();
        for (icon, priority) in [
            (CursorIcon::Default, 0.0),
            (CursorIcon::Pointer, 1.0),
            (CursorIcon::Grab, 2.0),
        ] {
            cursor.request_cursor(icon, priority);
        }
        cam.spawn((
            StyledCursorBundle {
                cursor,
                sprite: SpriteBundle {
                    texture: assets.load("images/ui/cursor_pointer3D_shadow.png"),
                    transform: Transform {
                        scale: Vec3::new(0.45, 0.45, 1.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: YELLOW.into(),
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            GamepadCursor::new(0),
        ));
    });

    commands.spawn(SplashScreen);
}
pub fn exit(mut app_exit_event_writer: EventWriter<AppExit>, quit: Query<Entity, Added<Quit>>) {
    for _ in &quit {
        app_exit_event_writer.send(AppExit::Success);
    }
}

/// Function to return camera will all appropriate settings
pub fn camera() -> impl Bundle {
    (
        MainUi,
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        InheritedVisibility::default(),
    )
}

pub fn create_root(commands: &mut Commands, r_size: (f32, f32)) {
    commands
        .spawn(SpatialBundle::default())
        .with_children(|route| {
            route
                .spawn((
                    UiTreeBundle::<MainUi>::from(UiTree::new2d("Main")),
                    SourceFromCamera,
                ))
                .with_children(|ui| {
                    let root = UiLink::<MainUi>::path("Root");
                    ui.spawn((root.clone(), UiLayout::window().size(r_size).pack::<Base>()));
                });
        });
}
pub fn button_click(mut events: EventReader<UiClickEvent>, mut ev_w: EventWriter<SelectEvent>) {
    for event in events.read() {
        ev_w.send(SelectEvent(event.clone()));
    }
}
