use bevy::prelude::*;

use crate::vectors::Vector2Int;

use super::{components::EffectType, particle_builder::ParticleBuilder, EffectSpawner};

pub fn particle_to_tile(world: &mut World, position : Vector2Int, effect: &EffectSpawner) {    
    if let EffectType::Particle{ id, duration } = &effect.effect_type {
        if let Some(mut particle_builder) = world.get_resource_mut::<ParticleBuilder>() {
            particle_builder.request( id.clone(), position, duration.clone() );
        }
    }
}