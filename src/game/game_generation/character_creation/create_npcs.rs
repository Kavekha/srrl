use bevy::prelude::*;

use crate::{raws::{apply_referenced_job, get_job_table, get_spawn_table, spawn_referenced_entity, RAWS}, vectors::Vector2Int};

use super::components::Npc;


 
pub fn create_npc(world: &mut World, key: &str, npc_spawning_position: Vector2Int) -> Option<Entity> {     
    let npc_entity = spawn_referenced_entity(&RAWS.lock().unwrap(), world, key, npc_spawning_position);
    match npc_entity {
        None => { 
            info!("Can't create npc.");
            return None },
        Some(entity) => {
            // On recupère un job si il y en a un.
            let job_table = get_job_table(&RAWS.lock().unwrap(), key);
            let job = job_table.roll();
            println!("create npc: job rolled is : {:?}", job);
            apply_referenced_job(&RAWS.lock().unwrap(), world, &job, entity);

            world.entity_mut(entity)
            .insert(Npc)         
            ;
            return Some(entity)
        }        
    }
}
