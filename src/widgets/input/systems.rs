use bevy::prelude::*;
use bevy_hui::prelude::{HtmlComponents, HtmlFunctions};
use bevy_hui_widgets::prelude::TextInput;

pub fn register_input(
    mut comps: HtmlComponents,
    assets: Res<AssetServer>,
    mut funcs: HtmlFunctions,
) {
    comps.register(
        "input",
        assets.load("embedded://bevy_codex/widgets/input/input.html"),
    );
    funcs.register(
        "notify_input_change",
        |In(entity), inputs: Query<&TextInput>| {
            let Ok(input) = inputs.get(entity) else {
                return;
            };
            info!("Input {entity} changed, new value: `{}`", input.value);
        },
    );
}
