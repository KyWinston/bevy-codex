use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Knob {
    pub index_tag: String,
    pub value: f64,
}

#[derive(Component)]
pub struct Rack {
    pub root_res: String,
    pub index_tag: String,
}

#[derive(Component, Clone)]
pub struct SliderUi;
