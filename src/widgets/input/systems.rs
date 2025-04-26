use bevy::{prelude::*, ui::FocusPolicy};
use bevy_hui::prelude::{HtmlComponents, HtmlFunctions, Tags};
use bevy_simple_text_input::{TextInput, TextInputInactive, TextInputPlaceholder, TextInputValue};

use crate::settings::resources::{GameSettings, SettingType, SettingsHandle};

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
            let Ok(_input) = inputs.get(entity) else {
                return;
            };
        },
    );
    funcs.register(
        "add_field",
        |In(entity),
         mut commands: Commands,
         input_node: Query<(Entity, &Tags)>,
         settings_handle: Res<SettingsHandle>,
         settings: Res<Assets<GameSettings>>| {
            for (ent, tags) in input_node.iter() {
                let mut index = 0;
                let mut input = "";
                for tag in tags.iter() {
                    if ent == entity && tag.0 == "index" {
                        index = tag.1.parse().expect("not a number");
                    }
                    if ent == entity && tag.0 == "input" {
                        input = tag.1;
                    }
                }
                if let Some(setting) = settings
                    .get(settings_handle.0.id())
                    .unwrap()
                    .get_setting(input.to_string(), Some(index))
                {
                    match setting {
                        SettingType::Text(s) => {
                            let field = commands
                                .spawn((
                                    TextInput,
                                    FocusPolicy::Block,
                                    TextInputValue(s.clone()),
                                    TextInputInactive(true),
                                    TextInputPlaceholder {
                                        value: s,
                                        ..default()
                                    },
                                ))
                                .id();
                            commands.entity(entity).insert_children(0, &[field]);
                        }
                        _ => {}
                    }
                }
            }
        },
    );
    funcs.register("focus_field", focus);
}

fn focus(In(entity): In<Entity>, mut text_input_query: Query<(&ChildOf, &mut TextInputInactive)>) {
    for (parent, mut inactive) in &mut text_input_query {
        if entity == **parent {
            inactive.0 = false;
        } else {
            inactive.0 = true;
        }
    }
}
