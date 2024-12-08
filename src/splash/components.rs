use bevy::prelude::*;
use std::time::Duration;


// Tag component used to tag entities added on the splash screen
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct SplashScreen;

// Tag component used to tag entities added on the splash screen
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct SplashUi;

#[derive(Clone)]
pub struct SplashItem {
    pub tint: Color,
    pub width: Val,
    pub height: Val,
    // pub ease_function: EaseMethod,
    pub duration: Duration,
    pub is_static: bool,
}

// Internal components for system logic
#[derive(Component)]
pub struct ClearSplash;
