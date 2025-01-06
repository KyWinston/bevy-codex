use bevy::{asset::embedded_asset, prelude::*};
use bevy_yarnspinner::{
    events::{DialogueCompleteEvent, DialogueStartEvent, PresentLineEvent},
    prelude::YarnSpinnerSystemSet,
};
use events::{SpeakerChangeEvent, TypewriterFinishedEvent};
use resources::Typewriter;
use systems::{despawn, present_line, send_finished_event, show_continue, spawn, write_text};

use crate::widgets::dialogue_view::DialogueViewSystemSet;

pub mod events;
pub mod resources;
pub mod systems;

pub fn typewriter_plugin(app: &mut App) {
    embedded_asset!(app, "dialogue_text.html");
    app.add_systems(
        Update,
        (
            spawn.run_if(on_event::<DialogueStartEvent>),
            present_line.run_if(resource_exists::<Typewriter>.and(on_event::<PresentLineEvent>)),
            send_finished_event.run_if(resource_exists::<Typewriter>),
            despawn.run_if(on_event::<DialogueCompleteEvent>),
            write_text.run_if(resource_exists::<Typewriter>),
            show_continue.run_if(resource_exists::<Typewriter>),
        )
            .chain()
            .after(YarnSpinnerSystemSet)
            .in_set(DialogueViewSystemSet),
    )
    .add_event::<SpeakerChangeEvent>()
    .register_type::<SpeakerChangeEvent>()
    .add_event::<TypewriterFinishedEvent>();
}
