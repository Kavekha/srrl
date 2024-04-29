use bevy::prelude::*;

use crate::{game::{combat::rules::VISIBILITY_RANGE_PLAYER, pieces::components::Npc, player::Player, visibility::components::View}, raws::{spawn_referenced_entity, RAWS}, vectors::Vector2Int};



pub fn create_npc(world: &mut World, npc_spawning_position: Vector2Int){
    
    let npc_entity = spawn_referenced_entity(&RAWS.lock().unwrap(), world, "ghoul", npc_spawning_position);
    match npc_entity {
        None => { info!("Can't create npc.")},
        Some(entity) => {

            world.entity_mut(entity)
            .insert(Npc)         
            ;
        }        
    }
}
