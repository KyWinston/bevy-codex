use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct StatusBarUi;

#[derive(Component, Default, Clone)]
pub struct StatusBar {
    pub label: String,
    pub color: Color,
    pub value: f32,
    pub frame: Option<Handle<Image>>,
    pub top_left: (f32, f32),
    pub bottom_right: (f32, f32),
}
