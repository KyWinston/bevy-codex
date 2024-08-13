use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct StatusBarUi;

#[derive(Component)]
pub struct StatusBar {
    pub label: String,
    pub color: Color,
    pub value: f32,
    pub top_left: (f32, f32),
    pub bottom_right: (f32, f32),
}
