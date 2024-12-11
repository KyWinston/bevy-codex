use bevy::prelude::*;
use crate::components::Quit;

pub fn exit(mut app_exit_event_writer: EventWriter<AppExit>, quit: Query<Entity, Added<Quit>>) {
    for _ in &quit {
        app_exit_event_writer.send(AppExit::Success);
    }
}
