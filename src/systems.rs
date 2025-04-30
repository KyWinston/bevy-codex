use crate::components::Quit;
use bevy::prelude::*;

pub fn exit(mut app_exit_event_writer: EventWriter<AppExit>, quit: Query<Entity, Added<Quit>>) {
    for _ in &quit {
        app_exit_event_writer.write(AppExit::Success);
    }
}


