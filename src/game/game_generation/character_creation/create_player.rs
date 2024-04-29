use bevy::prelude::*;

use crate::{game::{rules::VISIBILITY_RANGE_PLAYER, player::Player, visibility::components::View}, raws::{spawn_referenced_entity, RAWS}, vectors::Vector2Int};



pub fn create_player(world: &mut World, player_starting_position: Vector2Int){
    println!("Player: Starting position = {:?}", player_starting_position);

    //let kind = get_random_kind();    
    
    let playable_entity = spawn_referenced_entity(&RAWS.lock().unwrap(), world, "human", player_starting_position);
    
    match playable_entity {
        None => { panic!("Can't create player.")},         
        Some(player_entity) => {
            // Surcharge du Nom.
            let mut entity_ref = world.entity_mut(player_entity);
            let mut name = entity_ref.get_mut::<Name>().unwrap();       // REMEMBER : C'est comme ca qu'on GET un component depuis WORLD.
            name.set("The Shadowrunner");
 
            world.entity_mut(player_entity)
            .insert(Player)
            .insert(View { 
                visible_tiles: Vec::new(),
                range: VISIBILITY_RANGE_PLAYER
            })
            ;
        }               
    }
}