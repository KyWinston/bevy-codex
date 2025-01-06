use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_hui::prelude::{HtmlNode, TemplateProperties};
use bevy_yarnspinner::{
    events::{DialogueCompleteEvent, PresentOptionsEvent},
    prelude::{DialogueOption, DialogueRunner},
};

use crate::widgets::dialogue_view::{
    components::{DialogueRootNode, OptionButton, OptionsNode},
    dialogue_text::{events::TypewriterFinishedEvent, resources::Typewriter},
};

use super::{events::HasSelectedOptionEvent, resource::OptionSelection, NUMBER_KEYS, NUMPAD_KEYS};

pub fn create_options(
    option_selection: Res<OptionSelection>,
    mut commands: Commands,
    children: Query<&Children>,
    assets: Res<AssetServer>,
    mut options_node: Query<(Entity, &mut Visibility), With<OptionsNode>>,
) {
    let (entity, mut visibility) = options_node.single_mut();
    *visibility = Visibility::Hidden;
    if children.iter_descendants(entity).next().is_none() {
        let mut entity_commands = commands.entity(entity);
        spawn_options(&mut entity_commands, &option_selection.options, assets);
    }
}

fn spawn_options<'a, T>(entity_commands: &mut EntityCommands, options: T, assets: Res<AssetServer>)
where
    T: IntoIterator<Item = &'a DialogueOption>,
    <T as IntoIterator>::IntoIter: 'a,
{
    entity_commands.with_children(|parent| {
        for (i, option) in options.into_iter().enumerate() {
            parent
                .spawn((Button, OptionButton(option.id)))
                .with_children(|parent| {
                    parent.spawn((HtmlNode(assets.load("embedded://bevy_codex/widgets/dialogue_view/options_selection/single_option.html")),TemplateProperties::default().with("option_number", i.to_string().as_str()).with("option_text",&option.line.text)));
                });
        }
    });
}

pub fn show_options(
    mut typewriter_finished_event: EventReader<TypewriterFinishedEvent>,
    mut options_node: Query<&mut Visibility, With<OptionsNode>>,
) {
    for _event in typewriter_finished_event.read() {
        let mut visibility = options_node.single_mut();
        *visibility = Visibility::Inherited;
    }
}

pub fn select_option(
    keys: Res<ButtonInput<KeyCode>>,
    typewriter: Res<Typewriter>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    option_selection: Res<OptionSelection>,
    mut selected_option_event: EventWriter<HasSelectedOptionEvent>,
) {
    if !typewriter.is_finished() {
        return;
    }

    let mut selection = None;
    let key_to_option: HashMap<_, _> = NUMBER_KEYS
        .into_iter()
        .zip(NUMPAD_KEYS)
        .zip(option_selection.options.iter().map(|option| option.id))
        .collect();
    for ((num_key, numpad_key), option) in key_to_option {
        if keys.just_pressed(num_key) || keys.just_pressed(numpad_key) {
            selection = Some(option);
            break;
        }
    }

    let has_selected_id = selection.is_some();
    if let Some(id) = selection {
        for mut dialogue_runner in dialogue_runners.iter_mut() {
            dialogue_runner.select_option(id).unwrap();
        }
    }
    if has_selected_id {
        selected_option_event.send(HasSelectedOptionEvent);
    }
}

pub fn despawn_options(
    mut has_selected_option_event: EventReader<HasSelectedOptionEvent>,
    mut dialogue_complete_event: EventReader<DialogueCompleteEvent>,
    mut commands: Commands,
    mut options_node: Query<(Entity, &mut Visibility), With<OptionsNode>>,
    mut dialogue_node_text: Query<&mut Text, With<DialogueRootNode>>,
) {
    let should_despawn =
        !has_selected_option_event.is_empty() || !dialogue_complete_event.is_empty();
    if !should_despawn {
        return;
    }
    has_selected_option_event.clear();
    dialogue_complete_event.clear();
    commands.remove_resource::<OptionSelection>();
    let (entity, mut visibility) = options_node.single_mut();
    commands.entity(entity).despawn_descendants();
    *visibility = Visibility::Hidden;
    for mut text in dialogue_node_text.iter_mut() {
        *text = Text::default();
    }
}

pub fn present_options(mut commands: Commands, mut events: EventReader<PresentOptionsEvent>) {
    for event in events.read() {
        let option_selection = OptionSelection::from_option_set(&event.options);
        commands.insert_resource(option_selection);
    }
}
