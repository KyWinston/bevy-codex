use bevy::prelude::*;
// use bevy_codex::widgets::dialogue::events::StartDialogueEvent;
// use lyrebird::soundfont::resources::{SoundFont, SoundFonts};

// use crate::resources::SoundfontManifest;

// pub fn run_test_dialogue(
//     mut talk_ev: EventWriter<StartDialogueEvent>,
//     key: Res<ButtonInput<KeyCode>>,
// ) {
//     if key.just_released(KeyCode::KeyP) {
//         talk_ev.send(StartDialogueEvent);
//     }
// }

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}

// pub fn load_audio(mut soundfonts: ResMut<SoundFonts>, manifest: Res<SoundfontManifest>) {
//     for (_, item) in manifest.soundfonts.iter() {
//         println!("{:?}", item.name);
//         soundfonts.0.insert(
//             item.name.clone(),
//             SoundFont {
//                 name: item.name.clone(),
//                 handle: item.handle.clone(),
//                 target_chunk: item.target_chunk,
//                 volume: item.volume,
//             },
//         );
//     }
// }
