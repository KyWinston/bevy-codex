use bevy::{asset::embedded_asset, prelude::*};
use bevy_yarnspinner::{
    events::{DialogueCompleteEvent, DialogueStartEvent},
    prelude::{YarnProject, YarnSpinnerPlugin},
};
use continue_button::continue_button_plugin;
use dialogue_name::dialogue_name_plugin;
use dialogue_text::{
    events::DialogueContinueEvent,
    systems::{hide_dialog, show_dialog},
    typewriter_plugin,
};
use events::RunDialogueEvent;
use options_selection::option_selection_plugin;
use systems::{open_dialogue, setup};

use widgets::ui_assets_plugin;

pub mod components;
pub mod continue_button;
pub mod dialogue_name;
pub mod dialogue_text;
pub mod events;
pub mod options_selection;
pub mod systems;
pub mod widgets;

#[derive(Debug, Default)]
pub struct DialogueViewPlugin;

#[derive(Debug, Default, Clone, Copy, SystemSet, Eq, PartialEq, Hash)]
pub struct DialogueViewSystemSet;

impl Plugin for DialogueViewPlugin {
    fn build(&self, app: &mut App) {
        assert!(
            app.is_plugin_added::<YarnSpinnerPlugin>(),
            "YarnSpinnerPlugin must be added before DialogueViewPlugin"
        );
        embedded_asset!(app, "dialogue_view.html");
        app.add_event::<RunDialogueEvent>()
            .add_event::<DialogueStartEvent>()
            .add_event::<DialogueContinueEvent>()
            .add_event::<DialogueCompleteEvent>()
            .add_systems(PreStartup, setup)
            .add_systems(
                Update,
                (
                    show_dialog.run_if(on_event::<DialogueStartEvent>),
                    hide_dialog,
                    open_dialogue.run_if(resource_exists::<YarnProject>),
                ),
            )
            .add_plugins((
                ui_assets_plugin,
                dialogue_name_plugin,
                typewriter_plugin,
                option_selection_plugin,
                continue_button_plugin,
            ));
    }
}
