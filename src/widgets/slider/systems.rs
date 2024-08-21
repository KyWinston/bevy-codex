use bevy::prelude::*;
use bevy_lunex::prelude::*;
use bevy_settings::PersistSettings;

use super::components::{Knob, Rack, SliderUi};
use crate::settings::resources::Settings;

pub fn update_value(
    mut settings: ResMut<Settings>,
    mut ui_q: Query<(&Knob, &mut Style)>,
    mut rack_q: Query<&Rack>,
    mut ev: EventReader<Pointer<Drag>>,
    mut writer: EventWriter<PersistSettings>,
) {
    for e in ev.read() {
        if let Ok((knob, mut style)) = ui_q.get_mut(e.target) {
            style.left =
                Val::Px((e.pointer_location.position.x + e.event.delta.x).clamp(0.0, 500.0));
            for (_, value) in settings.sound_settings.iter_mut() {
                for _r in rack_q.iter_mut().filter(|p| p.index_tag == knob.index_tag) {
                    *value = knob.value;
                }
            }
            writer.send(PersistSettings);
        }
    }
}

pub fn build_slider(mut commands: Commands, query: Query<(Entity, &SliderUi), Added<SliderUi>>) {
    for (entity, _) in &query {
        commands
            .entity(entity)
            .insert(UiTreeBundle::<SliderUi>::from(UiTree::new2d("Slider")))
            .with_children(|_| {});
    }
}
