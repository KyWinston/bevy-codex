use bevy::{app::PluginGroupBuilder, prelude::*};
use button::ButtonPlugin;
use dialogue_view::DialogueViewPlugin;
// use list::ListPlugin;
// use panel::PanelPlugin;
// use status_bar::StatusBarPlugin;

pub mod button;
// pub mod list;
pub mod dialogue_view;
// pub mod panel;
// pub mod status_bar;

pub struct WidgetPlugins;

impl PluginGroup for WidgetPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            // .add(ListPlugin)
            // .add(PanelPlugin)
            .add(ButtonPlugin)
            // .add(StatusBarPlugin)
            .add(DialogueViewPlugin)
    }
}
