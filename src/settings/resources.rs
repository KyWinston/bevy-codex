use bevy::{ecs::system::Resource, utils::HashMap};

use serde::{Deserialize, Serialize};

#[derive(Resource, Default, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "bevy_settings::serde")]
pub struct Settings {
    pub sound_settings: HashMap<String, f64>,
}
