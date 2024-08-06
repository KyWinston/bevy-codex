use bevy::prelude::*;
use bevy_lunex::UiClickEvent;

#[derive(Event)]
pub struct SelectEvent(pub UiClickEvent);