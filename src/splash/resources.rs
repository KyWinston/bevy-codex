use bevy::prelude::*;

// use super::components::SplashScreen;

#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(pub Timer);

#[derive(Default, Clone, Resource)]
pub struct SplashScreenSkipable(pub bool, pub bool);

// #[derive(Default, Clone, Resource)]
// pub(crate) struct SplashScreens(pub Vec<SplashScreen>);

// #[derive(Resource)]
// pub(crate) struct MaxScreens<S>(pub(crate) u64, pub(crate) S, pub(crate) u64)
// where
//     S: States;
