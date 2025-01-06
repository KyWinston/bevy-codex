use bevy::prelude::*;
use bevy_yarnspinner::prelude::OptionId;


#[derive(Debug, Default, Component)]
pub struct DialogueRootNode;

#[derive(Debug, Default, Component)]
pub struct DialogueNameNode;

#[derive(Debug, Default, Component)]
pub struct DialogueTextNode;

#[derive(Debug, Default, Component)]
pub struct DialogueContinueNode;

#[derive(Debug, Default, Component)]
pub struct OptionsNode;

#[derive(Debug, Component)]
pub struct OptionButton(pub OptionId);
