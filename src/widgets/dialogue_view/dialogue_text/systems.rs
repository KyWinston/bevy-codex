use crate::widgets::dialogue_view::{
    components::{DialogueContinueNode, DialogueNameNode, DialogueRootNode, DialogueTextNode},
    options_selection::resource::OptionSelection,
};
use bevy::prelude::*;
use bevy_yarnspinner::events::{DialogueCompleteEvent, PresentLineEvent};

use super::{
    events::{SpeakerChangeEvent, TypewriterFinishedEvent},
    resources::Typewriter,
};

pub fn show_continue(
    typewriter: Res<Typewriter>,
    mut visibility: Query<&mut Visibility, With<DialogueContinueNode>>,
    mut typewriter_finished_event: EventReader<TypewriterFinishedEvent>,
) {
    for _event in typewriter_finished_event.read() {
        if !typewriter.last_before_options {
            if let Ok(mut visibility) = visibility.single_mut() {
                *visibility = Visibility::Inherited;
            }
        }
    }
}

pub fn despawn(mut commands: Commands) {
    commands.remove_resource::<Typewriter>();
}

pub fn spawn(mut commands: Commands) {
    commands.init_resource::<Typewriter>();
}

pub fn send_finished_event(
    mut events: EventWriter<TypewriterFinishedEvent>,
    typewriter: Res<Typewriter>,
    mut last_finished: Local<bool>,
) {
    if !typewriter.is_finished() {
        *last_finished = false;
    } else if !*last_finished {
        events.write(TypewriterFinishedEvent);
        *last_finished = true;
    }
}

pub fn write_text(
    mut children: Query<&mut Children, With<DialogueTextNode>>,
    mut texts: Query<(&mut Visibility, &mut Text), Without<DialogueRootNode>>,
    mut typewriter: ResMut<Typewriter>,
    option_selection: Option<Res<OptionSelection>>,
    mut speaker_change_events: EventWriter<SpeakerChangeEvent>,
    mut root_visibility: Query<&mut Visibility, With<DialogueRootNode>>,
) {
    if let Ok(child) = children.single_mut() {
        if let Ok((mut vis, mut text)) = texts.get_mut(child[0]) {
            if typewriter.last_before_options && option_selection.is_none() {
                *text = default();
                return;
            }
            if typewriter.is_finished() {
                return;
            }
            if !typewriter.last_before_options {
                *root_visibility.single_mut().unwrap() = Visibility::Inherited;
            }
            typewriter.update_current_text();
            if typewriter.is_finished() {
                if let Some(name) = typewriter.character_name.as_deref() {
                    speaker_change_events.write(SpeakerChangeEvent {
                        character_name: name.to_string(),
                        speaking: false,
                    });
                }
            }

            let current_text = &typewriter.current_text;
            let rest = typewriter.graphemes_left.join("");
            *text = Text(current_text.to_string());
            *vis = Visibility::Inherited;

            if let Ok((mut vis, mut remaining)) = texts.get_mut(child[1]) {
                *vis = Visibility::Hidden;
                *remaining = Text(rest.to_string());
            }
        }
    }
}
pub fn show_dialog(mut root_node: Query<&mut Visibility, With<DialogueRootNode>>) {
    if let Ok(mut visibility) = root_node.single_mut() {
        *visibility = Visibility::Visible;
    }
}

pub fn hide_dialog(
    mut root_visibility: Query<&mut Visibility, With<DialogueRootNode>>,
    mut dialogue_complete_events: EventReader<DialogueCompleteEvent>,
) {
    if !dialogue_complete_events.is_empty() {
        *root_visibility.single_mut().unwrap() = Visibility::Hidden;
        dialogue_complete_events.clear();
    }
}

pub fn present_line(
    mut line_events: EventReader<PresentLineEvent>,
    mut speaker_change_events: EventWriter<SpeakerChangeEvent>,
    mut typewriter: ResMut<Typewriter>,
    mut name_node: Query<&Children, With<DialogueNameNode>>,
    mut texts: Query<&mut Text>,
) {
    for event in line_events.read() {
        let name = if let Some(name) = &event.line.character_name() {
            speaker_change_events.write(SpeakerChangeEvent {
                character_name: name.to_string(),
                speaking: true,
            });
            name.to_string()
        } else {
            String::new()
        };
        if let Ok(child) = name_node.single_mut() {
            if let Ok(mut node) = texts.get_mut(child[0]) {
                node.0 = name;
                typewriter.set_line(&event.line);
            }
        };
    }
}
