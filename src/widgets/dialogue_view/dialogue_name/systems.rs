use crate::widgets::dialogue_view::{
    components::DialogueNameNode, dialogue_text::events::SpeakerChangeEvent,
};
use bevy::prelude::*;

pub fn update_speaker(
    mut ev: EventReader<SpeakerChangeEvent>,
    mut children: Query<(&mut Visibility, &Children), With<DialogueNameNode>>,
    mut texts: Query<&mut Text>,
) {
    for ev in ev.read() {
        if let Ok((mut vis, child)) = children.single_mut() {
            if let Ok(mut text) = texts.get_mut(child[0]) {
                text.0 = ev.character_name.clone();
                *vis = Visibility::Inherited;
            }
        }
    }
}
