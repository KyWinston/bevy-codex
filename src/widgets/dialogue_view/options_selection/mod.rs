use bevy::{asset::embedded_asset, prelude::*};
use bevy_yarnspinner::{events::PresentOptionsEvent, prelude::DialogueOption};
use events::HasSelectedOptionEvent;
use resource::OptionSelection;
use systems::{create_options, despawn_options, present_options, select_option, show_options};

use super::{dialogue_text::systems::despawn, DialogueViewSystemSet};

pub mod events;
pub mod resource;
pub mod systems;

pub fn option_selection_plugin(app: &mut App) {
    embedded_asset!(app, "dialogue_options.html");
    embedded_asset!(app, "single_option.html");

    app.add_systems(
        Update,
        (
            create_options.run_if(resource_added::<OptionSelection>),
            show_options,
            present_options.run_if(on_event::<PresentOptionsEvent>),
            select_option
                .run_if(resource_exists::<OptionSelection>)
                .before(despawn),
            despawn_options,
        )
            .chain()
            .in_set(DialogueViewSystemSet),
    )
    .add_event::<HasSelectedOptionEvent>();
}

impl OptionSelection {
    pub fn from_option_set<'a>(options: impl IntoIterator<Item = &'a DialogueOption>) -> Self {
        let options = options
            .into_iter()
            .filter(|o| o.is_available)
            .cloned()
            .collect();
        Self { options }
    }
}

const NUMBER_KEYS: [KeyCode; 9] = [
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::Digit5,
    KeyCode::Digit6,
    KeyCode::Digit7,
    KeyCode::Digit8,
    KeyCode::Digit9,
];

const NUMPAD_KEYS: [KeyCode; 9] = [
    KeyCode::Numpad1,
    KeyCode::Numpad2,
    KeyCode::Numpad3,
    KeyCode::Numpad4,
    KeyCode::Numpad5,
    KeyCode::Numpad6,
    KeyCode::Numpad7,
    KeyCode::Numpad8,
    KeyCode::Numpad9,
];
