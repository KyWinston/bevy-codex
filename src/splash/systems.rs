use bevy::{
    input::{gamepad::GamepadEvent, keyboard::KeyboardInput, mouse::MouseButtonInput},
    prelude::*,
    state::state::FreelyMutableState,
};
use bevy_hui::prelude::HtmlNode;

use super::{components::ClearSplash, resources::SplashScreenSkipable};

pub fn create_splash(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.spawn(HtmlNode(assets.load("pages/splash.html")));
}

pub fn splash_skip<S: FreelyMutableState>(
    mut kbd: EventReader<KeyboardInput>,
    mut mouse: EventReader<MouseButtonInput>,
    mut gamepad: EventReader<GamepadEvent>,
    mut touch: EventReader<TouchInput>,
    brands: Query<(Entity, &Node, &ClearSplash)>,
    skipable: Res<SplashScreenSkipable>,
) {
    if brands.is_empty() || !skipable.0 {
        return;
    }

    let mut _done = false;

    if !skipable.1 {
        use bevy::input::{touch::TouchPhase, ButtonState};

        _done = kbd.read().any(|ev| ev.state == ButtonState::Pressed)
            || mouse.read().any(|ev| ev.state == ButtonState::Pressed)
            || touch.read().any(|ev| ev.phase == TouchPhase::Started);

        for ev in gamepad.read() {
            if let GamepadEvent::Button(_) = ev {
                _done = true;
            }
        }
    }
}
