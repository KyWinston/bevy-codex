use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};

// use leafwing_manifest::{
//     asset_state::SimpleAssetState,
//     plugin::{ManifestPlugin, RegisterManifest},
// };
// use lyrebird::LyrebirdPlugin;
// use resources::SoundfontManifest;
use systems::setup;

pub mod resources;
pub mod systems;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::WARN,
                    filter: "bevy_midi=debug".to_string(),
                    ..default()
                })
                .build(),
            // LyrebirdPlugin,
            // ManifestPlugin::<SimpleAssetState> {
            //     automatically_advance_states: true,
            //     _phantom: std::marker::PhantomData,
            // },
        ))
        // .register_manifest::<SoundfontManifest>("soundfonts/soundfonts.ron")
        .add_systems(Startup, setup)
        // .add_systems(OnEnter(SimpleAssetState::Ready), load_audio)
        // .add_systems(Update, run_test_dialogue)
        .run();
}
