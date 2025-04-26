use bevy::prelude::*;
use bevy_yarnspinner::prelude::DialogueRunner;

use crate::widgets::dialogue_view::{
    components::{DialogueContinueNode, DialogueRootNode},
    dialogue_text::resources::Typewriter,
    options_selection::resource::OptionSelection,
};

pub fn continue_dialogue(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    mut typewriter: ResMut<Typewriter>,
    option_selection: Option<Res<OptionSelection>>,
    mut continue_visibility: Query<
        &mut Visibility,
        (With<DialogueContinueNode>, Without<DialogueRootNode>),
    >,
) {
    let explicit_continue = keys.just_pressed(KeyCode::Space)
        || keys.just_pressed(KeyCode::Enter)
        || mouse_buttons.just_pressed(MouseButton::Left)
        || touches.any_just_pressed();
    if explicit_continue && !typewriter.is_finished() {
        typewriter.fast_forward();
        return;
    }

    if (explicit_continue || typewriter.last_before_options) && option_selection.is_none() {
        for mut dialogue_runner in dialogue_runners.iter_mut() {
            if !dialogue_runner.is_waiting_for_option_selection() && dialogue_runner.is_running() {
                dialogue_runner.continue_in_next_update();
                *continue_visibility.single_mut().unwrap() = Visibility::Hidden;
            }
        }
    }
}
