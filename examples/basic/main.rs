use bevy::prelude::*;
use bevy_codex::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiScreensPlugin::default()))
        .run();
}
