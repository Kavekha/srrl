use bevy::prelude::*;

use crate::{game::{effects::add_effect, game_generation::character_creation::components::Health, tileboard::components::BoardPosition}, vectors::Vector2Int};

use super::{components::{EffectType, Targets}, EffectSpawner, EFFECT_QUEUE};


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

pub fn inflict_damage(
    world: &mut World, 
    damage: &EffectSpawner, 
    target: Entity) {

    let mut add_blood = false;  // C'est merdique, mais faire ce code dans le damage effect provoque des problemes de borrow.
    let mut health_q = world.query::<&mut Health>();
    if let Ok(mut health) = health_q.get_mut(world, target) {
        println!("Target {:?} had {:?} hp.", target, health.current);
        if let EffectType::Damage{amount} = damage.effect_type {
            health.current -= amount;
            add_blood = true;               
        }
        println!("Now target {:?} has {:?} hp!", target, health.current);
    }

    if add_blood {
        if let Some(blood_position) = entity_position(world, target) {
            add_effect(None, EffectType::Bloodstain, Targets::Tile{ target:blood_position });
        }
    }
}

pub fn bloodstain(
    world: &mut World, 
    position : Vector2Int
) {
    println!("ADD blood on floor.");    // TODO  !
}

pub fn entity_position(
    world: &mut World, 
    target: Entity
) -> Option<Vector2Int> {    
    let mut position_q = world.query::<&BoardPosition>();
    if let Ok(position) = position_q.get(world, target) {
       return Some(position.v);
    }
    None
}