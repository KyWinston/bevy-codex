use bevy::{asset::embedded_asset, prelude::*};
use bevy_yarnspinner::prelude::YarnSpinnerSystemSet;
use systems::continue_dialogue;

use crate::widgets::dialogue_view::{dialogue_text::{resources::Typewriter, systems::spawn}, DialogueViewSystemSet};

pub mod systems;

pub fn continue_button_plugin(app: &mut App) {
    embedded_asset!(app, "continue_button.html");
    app.add_systems(
        Update,
        continue_dialogue
            .run_if(resource_exists::<Typewriter>)
            .after(YarnSpinnerSystemSet)
            .after(spawn)
            .in_set(DialogueViewSystemSet),
    );
}
