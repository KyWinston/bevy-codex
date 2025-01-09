use bevy::prelude::*;
use bevy_hui::prelude::{HtmlComponents, UiTarget};
use bevy_hui_widgets::prelude::SliderChangedEvent;

pub fn register_slider(mut comps: HtmlComponents, assets: Res<AssetServer>) {
    comps.register(
        "slider",
        assets.load("embedded://bevy_codex/widgets/slider/slider.html"),
    );
}

// -----------------
// example, custom user extension, update a value display of a slider

pub fn update_slider_target_text(
    mut events: EventReader<SliderChangedEvent>,
    targets: Query<&UiTarget>,
    mut texts: Query<&mut Text>,
) {
    for event in events.read() {
        let Ok(target) = targets.get(event.slider) else {
            continue;
        };

        let Ok(mut text) = texts.get_mut(**target) else {
            continue;
        };

        text.0 = format!("{:.2}", event.value);
    }
}
