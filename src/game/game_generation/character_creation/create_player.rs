use bevy::prelude::*;

use crate::{game::{player::Player, rules::VISIBILITY_RANGE_PLAYER, visibility::components::View}, menu_builders::menus::menu_char_selection::components::PlayerCreation, raws::{apply_referenced_job, spawn_referenced_entity, RAWS}, vectors::Vector2Int};



pub fn create_player(world: &mut World, player_starting_position: Vector2Int){
    println!("Player: Starting position = {:?}", player_starting_position);

    let Some(player_creation) = world.get_resource::<PlayerCreation>() else { panic!("No player to create.") };
    let player_kind = player_creation.kind.0.clone();   // Reference kind
    let player_job = player_creation.job.0.clone(); //Reference job


    let playable_entity = spawn_referenced_entity(&RAWS.lock().unwrap(), world, &player_kind, player_starting_position);
    
    match playable_entity {
        None => { panic!("Can't create player.")},         
        Some(player_entity) => {
            // Surcharge du Nom.
            let mut entity_ref = world.entity_mut(player_entity);
            let mut name = entity_ref.get_mut::<Name>().unwrap();       // REMEMBER : C'est comme ca qu'on GET un component depuis WORLD.
            name.set("The Shadowrunner");

            // 0.21i : can be selected.
            println!("Job reference for player is : {:?}", player_job);
            apply_referenced_job(&RAWS.lock().unwrap(), world, &player_job, player_entity);
 
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