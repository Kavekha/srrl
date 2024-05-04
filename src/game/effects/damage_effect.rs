use bevy::prelude::*;

use crate::game::{effects::components::EffectType, game_generation::character_creation::components::Health};

use super::{add_effect, components::Targets, targeting::entity_position, EffectSpawner};


pub fn inflict_damage(
    world: &mut World, 
    damage: &EffectSpawner, 
    target: Entity) {

    let mut get_damaged = false;  // C'est merdique, mais faire ce code dans le damage effect provoque des problemes de borrow.
    let mut get_killed = false;

    let mut health_q = world.query::<&mut Health>();
    if let Ok(mut health) = health_q.get_mut(world, target) {
        println!("Target {:?} had {:?} hp.", target, health.current);
        if let EffectType::Damage{amount} = damage.effect_type {
            health.current -= amount;
            get_damaged = true;    
            if health.current < 1 {
                get_killed = true;
            }           
        }
        println!("Now target {:?} has {:?} hp!", target, health.current);
    }

    if get_damaged {
        if let Some(target_position) = entity_position(world, target) {
            add_effect(None, EffectType::Bloodstain, Targets::Tile{ target:target_position });
            add_effect(None, EffectType::Particle { id: "hit_punch_blood".to_string(), duration: 1.0 }, Targets::Tile{ target:target_position });   // TO CHANGE : Duration a prendre en compte, id a generer autrement.
        }       
    }
    if get_killed {
        add_effect(damage.creator, EffectType::EntityDeath, Targets::Single{target})
    }
}