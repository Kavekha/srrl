use bevy::prelude::*;

use crate::{raws::{spawn_referenced_entity, RAWS}, vectors::Vector2Int};

use super::components::Npc;



pub fn create_npc(world: &mut World, key: &str, npc_spawning_position: Vector2Int) -> Option<Entity> { 
    
    let npc_entity = spawn_referenced_entity(&RAWS.lock().unwrap(), world, key, npc_spawning_position);
    match npc_entity {
        None => { 
            info!("Can't create npc.");
            return None },
        Some(entity) => {
            world.entity_mut(entity)
            .insert(Npc)         
            ;
            return Some(entity)
        }        
    }
}
