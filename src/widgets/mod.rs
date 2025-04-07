use bevy::{app::PluginGroupBuilder, prelude::*};
use button::ButtonPlugin;
use card::CardPlugin;
use dialogue_view::DialogueViewPlugin;
use input::InputFieldPlugin;
use panel::PanelPlugin;
use slider::SliderPlugin;
// use list::ListPlugin;
// use panel::PanelPlugin;
use status_bar::StatusBarPlugin;

pub(crate) mod button;
// pub mod list;
pub(crate) mod card;
pub(crate) mod input;
pub(crate) mod dialogue_view;
pub(crate) mod panel;
pub(crate) mod slider;
pub(crate) mod status_bar;

pub(crate) struct WidgetPlugins;

impl PluginGroup for WidgetPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CardPlugin)
            .add(InputFieldPlugin)
            .add(PanelPlugin)
            .add(SliderPlugin)
            .add(ButtonPlugin)
            .add(StatusBarPlugin)
            .add(DialogueViewPlugin)
    }
}
