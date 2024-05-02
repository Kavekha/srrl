use bevy::prelude::*;

use crate::{game::tileboard::components::BoardPosition, vectors::Vector2Int};

use super::components::{EffectType, NewEffectEvent, Targets};


pub fn run_effects_queue(
    mut ev_effect: EventReader<NewEffectEvent>,
    position_q: Query<(Entity, &BoardPosition)>
) {
    for effect in ev_effect.read() {
        match &effect.targets {
            Targets::Single { target } => affect_entity(effect, *target),
            Targets::List { target } => target.iter().for_each(|entity| affect_entity(effect, *entity)),
            Targets::Tile { target } => affect_tile(effect, *target, &position_q),
        }
    }
}

pub fn affect_tile(
    effect: &NewEffectEvent,
    target: Vector2Int,
    position_q: &Query<(Entity, &BoardPosition)>
) {
    if tile_effect_hits_entities(&effect) {
        for (entity, position) in position_q.iter() {
            if position.v == target {
                affect_entity(effect, entity);
            }
        }
    }
    // TODO: Run the effect
}

pub fn tile_effect_hits_entities(
    effect: &NewEffectEvent
)-> bool {
    match effect.effect_type {
        EffectType::Damage { .. } => return true,
        _ => return false
    }
}

fn affect_entity(
    effect: &NewEffectEvent,
    target: Entity
) {
    match effect.effect_type {
        EffectType::Damage { .. } => inflict_damage(effect, target),
    }
}

pub fn inflict_damage(
    effect: &NewEffectEvent,
    target: Entity
) {
    if let EffectType::Damage{amount} = effect.effect_type {
        //pool.hit_points.current -= amount;
    }
}