use std::{collections::VecDeque, sync::Mutex};

use bevy::prelude::*;

use self::{components::{EffectType, Targets}, effect_systems::run_effects_queue};

use super::states::GameState;


pub mod components;
mod effect_systems;





pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app

        .add_systems(Update, run_effects_queue.run_if(not(in_state(GameState::Unavailable))))
        ;
    }
}



lazy_static! {
    pub static ref EFFECT_QUEUE : Mutex<VecDeque<EffectSpawner>> = Mutex::new(VecDeque::new());
}

pub struct EffectSpawner {
    pub creator : Option<Entity>,
    pub effect_type : EffectType,
    pub targets : Targets
}

pub fn add_effect(creator : Option<Entity>, effect_type: EffectType, targets : Targets) {
    println!("Effect added");
    EFFECT_QUEUE
        .lock()
        .unwrap()
        .push_back(EffectSpawner{
            creator,
            effect_type,
            targets
        });
}