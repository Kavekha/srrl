use bevy::prelude::*;

use crate::{game::{combat::rules::VISIBILITY_RANGE_PLAYER, pieces::components::{Npc, Occupier}, player::Player, visibility::components::View}, raws::{spawn_named_kind, RAWS}, vectors::Vector2Int};



pub fn create_player(world: &mut World, player_starting_position: Vector2Int){
    println!("Player: Starting position = {:?}", player_starting_position);

    //let kind = get_random_kind();    
    
    let playable_entity = spawn_named_kind(&RAWS.lock().unwrap(), world, "human", player_starting_position);
    
    match playable_entity {
        None => { panic!("Can't create player.")},         
        Some(player_entity) => {

            world.entity_mut(player_entity)
            .insert(Player)
            .insert(Occupier)
            .insert(View { 
                visible_tiles: Vec::new(),
                range: VISIBILITY_RANGE_PLAYER
            })
            ;
        }   
            
    }
}

pub fn create_npc(world: &mut World, npc_spawning_position: Vector2Int){
    
    let npc_entity = spawn_named_kind(&RAWS.lock().unwrap(), world, "ghoul", npc_spawning_position);
    match npc_entity {
        None => { info!("Can't create npc.")},
        Some(entity) => {

            world.entity_mut(entity)
            .insert(Npc)       
            .insert(Occupier)     
            ;
        }        
    }
}
