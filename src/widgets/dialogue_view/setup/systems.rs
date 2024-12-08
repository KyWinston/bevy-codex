use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_hui::prelude::HtmlNode;
use bevy_yarnspinner::prelude::DialogueOption;

// use crate::widgets::dialogue_view::widgets::CONTINUE_INDICATOR;

pub fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(HtmlNode(assets.load("widgets/dialogue.html")));
}

pub fn fmt_name(name: &str) -> Name {
    Name::new(format!("Yarn Spinner example dialogue view node: {name}"))
}

// pub fn create_dialog_text(text: impl Into<String>, invisible: impl Into<String>) -> Text {
//     Text::from_sections([
//         TextSection {
//             value: text.into(),
//             style: text_standard(),
//         },
//         TextSection {
//             value: invisible.into(),
//             style: TextStyle {
//                 color: Color::NONE,
//                 ..text_standard()
//             },
//         },
//     ])
// }

// pub fn spawn_options<'a, T>(entity_commands: &mut EntityCommands, options: T)
// where
//     T: IntoIterator<Item = &'a DialogueOption>,
//     <T as IntoIterator>::IntoIter: 'a,
// {
//     entity_commands.with_children(|parent| {
//         for (i, option) in options.into_iter().enumerate() {
//             parent
//                 .spawn((
//                     fmt_name("option button"),
//                     ButtonBundle {
//                         style: Style {
//                             justify_content: JustifyContent::FlexStart,
//                             ..default()
//                         },
//                         image: UiImage::default().with_color(Color::NONE),
//                         ..default()
//                     },
//                     OptionButton(option.id),
//                 ))
//                 .with_children(|parent| {
//                     let sections = [
//                         TextSection {
//                             value: format!("{}: ", i + 1),
//                             style: option_id(),
//                         },
//                         TextSection {
//                             value: option.line.text.clone(),
//                             style: option_text(),
//                         },
//                     ];

//                     parent.spawn((
//                         fmt_name("option text"),
//                         TextBundle::from_sections(sections).with_style(style::options()),
//                         Label,
//                     ));
//                 });
//         }
//     });
// }
