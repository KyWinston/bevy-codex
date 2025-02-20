use bevy::{prelude::*, reflect::Map};
use bevy_hui::prelude::Tags;

#[derive(Event)]
pub struct UpdateUiTextEvent(pub String, pub String);

pub(crate) fn update_ui_text(
    mut text_ev: EventReader<UpdateUiTextEvent>,
    mut ui_text: Query<(&mut Text, &Tags)>,
) {
    for ev in text_ev.read() {
        for (mut text, tags) in ui_text.iter_mut() {
            for tag in tags.iter() {
                if tag.0 == "label" && *tag.1 == ev.0 {
                    text.0 = ev.1.clone();
                }
            }
        }
    }
}
