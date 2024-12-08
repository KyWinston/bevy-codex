use bevy::prelude::*;

use super::resources::VocalBark;

#[derive(Debug, Eq, PartialEq, Hash, Reflect, Event)]
pub struct TypewriterFinishedEvent;

#[derive(Event)]
pub struct VBplayEvent(pub VocalBark);
