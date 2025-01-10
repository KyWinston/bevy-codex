use bevy::prelude::*;
use bevy_hui::prelude::{HtmlComponents, HtmlFunctions, UiTarget};
use bevy_hui_widgets::prelude::{Slider, SliderChangedEvent};

pub fn register_slider(
    mut comps: HtmlComponents,
    assets: Res<AssetServer>,
    mut funcs: HtmlFunctions,
) {
    comps.register(
        "slider",
        assets.load("embedded://bevy_codex/widgets/slider/slider.html"),
    );
    funcs.register(
        "notify_slider_change",
        |In(entity), sliders: Query<&Slider>| {
            let Ok(slider) = sliders.get(entity) else {
                return;
            };
            info!("Slider {entity} changed, new value: {:.2}", slider.value);
        },
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
