use bevy::{color::palettes::css::WHITE, prelude::*, sprite::Anchor};

use crate::widgets::{button::components::CustomButtonRef, status_bar::components::StatusBar};

#[derive(Component, Clone)]
pub struct PanelUi;

#[derive(Component)]
pub struct PanelText;

#[derive(Component)]
pub struct Panel<T> {
    pub text: Option<String>,
    pub texture: Option<Handle<Image>>,
    pub color: Color,
    pub text_alignment: Anchor,
    pub content: Vec<T>,
}

impl Default for Panel<CustomButtonRef> {
    fn default() -> Self {
        Self {
            text: None,
            texture: None,
            color: WHITE.into(),
            text_alignment: Anchor::TopCenter,
            content: vec![],
        }
    }
}


impl Default for Panel<StatusBar> {
    fn default() -> Self {
        Self {
            text: None,
            texture: None,
            color: WHITE.into(),
            text_alignment: Anchor::CenterLeft,
            content: vec![],
        }
    }
}
