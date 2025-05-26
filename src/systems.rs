use crate::components::Quit;
use bevy::prelude::*;
use bevy_hui::prelude::{HtmlStyle, UiId};

pub fn exit(mut app_exit_event_writer: EventWriter<AppExit>, quit: Query<Entity, Added<Quit>>) {
    for _ in &quit {
        app_exit_event_writer.write(AppExit::Success);
    }
}

pub fn clear_default_styles(mut commands: Commands, nodes: Query<Entity, With<HtmlStyle>>) {
    for node in nodes.iter() {
        commands.entity(node).remove::<HtmlStyle>();
    }
}

pub fn assign_node_ids(mut commands: Commands, nodes: Query<(Entity, &UiId), With<Node>>) {
    for (node, id) in nodes.iter() {
        let name = id.id().clone();
        commands
            .entity(node)
            .insert(Name::new(name))
            .remove::<UiId>();
    }
}
