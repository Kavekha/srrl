use std::{collections::VecDeque, sync::Mutex};

use bevy::prelude::*;

use self::{components::{EffectType, Targets}, effect_engine::run_effects_queue, particle_builder::{particle_spawning, ParticleBuilder}};

use super::states::GameState;


pub mod components;
mod effect_engine;
pub mod damage_effect;
mod particle_builder;
mod particle_effect;
mod targeting;
mod death_effect;




pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app

        .insert_resource(ParticleBuilder::new())

        .add_systems(Update, run_effects_queue.run_if(not(in_state(GameState::Unavailable))))
        .add_systems(Update, particle_spawning)
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