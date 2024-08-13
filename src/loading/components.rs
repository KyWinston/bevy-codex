use bevy::prelude::*;

#[derive(Component)]
pub struct Loading(pub Option<String>);

#[derive(Component, Clone)]
pub struct LoadingUi;
