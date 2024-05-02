use bevy::prelude::*;

use self::{components::NewEffectEvent, effect_systems::run_effects_queue};


mod components;
mod effect_systems;





pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<NewEffectEvent>()

        .add_systems(Update, run_effects_queue.run_if(on_event::<NewEffectEvent>()))
        ;
    }
}