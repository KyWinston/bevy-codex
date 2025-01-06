use bevy::prelude::*;
use bevy_hui::prelude::{HtmlComponents, TemplateProperties};
use bevy_yarnspinner::prelude::YarnProject;

use super::{
    components::{
        DialogueContinueNode, DialogueNameNode, DialogueRootNode, DialogueTextNode, OptionsNode,
    },
    events::RunDialogueEvent,
};

pub fn setup(assets: Res<AssetServer>, mut html_comps: HtmlComponents) {
    html_comps.register_with_spawn_fn(
        "dview",
        assets.load("embedded://bevy_codex/widgets/dialogue_view/dialogue_view.html"),
        |mut cmd| {
            cmd.insert((
                DialogueRootNode,
                Visibility::Hidden,
                TemplateProperties::default(),
            ));
        },
    );

    html_comps.register_with_spawn_fn(
        "dname",
        assets.load("embedded://bevy_codex/widgets/dialogue_view/dialogue_name/dialogue_name.html"),
        |mut cmd| {
            cmd.insert((DialogueNameNode, Visibility::Inherited));
        },
    );

    html_comps.register_with_spawn_fn(
        "dtext",
        assets.load("embedded://bevy_codex/widgets/dialogue_view/dialogue_text/dialogue_text.html"),
        |mut cmd| {
            cmd.insert((DialogueTextNode, Visibility::Inherited));
        },
    );

    html_comps.register_with_spawn_fn(
        "cbutton",
        assets.load(
            "embedded://bevy_codex/widgets/dialogue_view/continue_button/continue_button.html",
        ),
        |mut cmd| {
            cmd.insert((DialogueContinueNode, Visibility::Inherited));
        },
    );

    html_comps.register_with_spawn_fn(
        "dopt",
        assets.load(
            "embedded://bevy_codex/widgets/dialogue_view/options_selection/dialogue_options.html",
        ),
        |mut cmd| {
            cmd.insert((OptionsNode, Visibility::Inherited));
        },
    );
}

pub fn fmt_name(name: &str) -> Name {
    Name::new(format!("Yarn Spinner example dialogue view node: {name}"))
}

pub fn run_dialog(
    mut commands: Commands,
    mut run_ev: EventReader<RunDialogueEvent>,
    project: Res<YarnProject>,
) {
    for ev in run_ev.read() {
        let mut dialogue_runner = project.create_dialogue_runner();
        dialogue_runner.start_node(ev.0.clone());
        commands.spawn(dialogue_runner);
    }
}
