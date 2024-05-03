use bevy::prelude::*;

use crate::{game::tileboard::components::BoardPosition, vectors::Vector2Int};

use super::{components::{EffectType, Targets}, damage_effect::inflict_damage, particle_effect::particle_to_tile, targeting::entity_position, EffectSpawner, EFFECT_QUEUE};


pub fn run_effects_queue(world : &mut World) {
    loop {
        let effect : Option<EffectSpawner> = EFFECT_QUEUE.lock().unwrap().pop_front();
        if let Some(effect) = effect {
            target_applicator(world, &effect); 
        } else {
            break;
        }
    }
}

pub fn target_applicator(
    world: &mut World,
    effect: &EffectSpawner
) {
    println!("Target applicator!");
    match &effect.targets {
        Targets::Single { target } => affect_entity(world, effect, *target),
        Targets::List { target } => target.iter().for_each(|entity| affect_entity(world, effect, *entity)),
        Targets::Tile { target } => affect_tile(world, effect, *target),      
    }
}


fn affect_entity(
    world: &mut World,
    effect: &EffectSpawner,
    target: Entity,
) {
    match effect.effect_type {
        EffectType::Damage { .. } => inflict_damage(world, effect, target), 
        EffectType::Bloodstain => if let Some(position) = entity_position(world, target) { },   // TODO : Bloodstain.
        EffectType::Particle { .. } => if let Some(position) = entity_position(world, target) { particle_to_tile(world, position, &effect) }
        _ => {},
    }
}

pub fn affect_tile(
    world: &mut World, 
    effect: &EffectSpawner,
    position: Vector2Int,
) {
    if tile_effect_hits_entities(&effect) {
        let mut position_q = world.query::<(Entity, &BoardPosition)>();
        let mut affected_entities = Vec::new();
        for (entity, board_position) in position_q.iter(&world) {
            if board_position.v == position { 
                affected_entities.push(entity); 
            }
        }
        for entity in affected_entities {
            affect_entity(world, effect, entity)
        }        
    }
    match &effect.effect_type {
        EffectType::Bloodstain => bloodstain(world, position),
        EffectType::Particle { .. } => particle_to_tile(world, position, &effect),
        _ => {}
    }
}

pub fn tile_effect_hits_entities(
    effect: &EffectSpawner
)-> bool {
    match effect.effect_type {
        EffectType::Damage { .. } => return true,
        _ => return false
    }
}


pub fn bloodstain(
    world: &mut World, 
    position : Vector2Int
) {
    println!("ADD blood on floor.");    // TODO  !
}

