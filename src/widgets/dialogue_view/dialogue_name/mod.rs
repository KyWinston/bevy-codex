use bevy::{asset::embedded_asset, prelude::*};
use bevy_yarnspinner::prelude::YarnSpinnerSystemSet;
use systems::update_speaker;

use crate::widgets::dialogue_view::DialogueViewSystemSet;

pub const UNNAMED: &str = "unnamed";

pub mod systems;

pub fn dialogue_name_plugin(app: &mut App) {
    embedded_asset!(app, "dialogue_name.html");
    app.add_systems(
        Update,
        update_speaker
            .after(YarnSpinnerSystemSet)
            .in_set(DialogueViewSystemSet),
    );
}
