use bevy::prelude::*;
use rand::Rng;

use crate::{game::{player::Player, rules::VISIBILITY_RANGE_PLAYER, visibility::components::View}, menu_builders::menus::menu_char_selection::components::PlayerCreation, raws::{apply_referenced_job, spawn_referenced_entity, RAWS}, vectors::Vector2Int};



pub fn create_player(world: &mut World, player_starting_position: Vector2Int){
    println!("Player: Starting position = {:?}", player_starting_position);

    let Some(player_creation) = world.get_resource::<PlayerCreation>() else { panic!("No player to create.") };
    let player_kind = player_creation.kind.clone();

    //let kind = get_random_kind();  
    /* 
    let kind:String;  
    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(1..6); // Exclusif 
    println!("rand is {:?}", rand);
    kind = match rand {
        1 => "human".to_string(),
        2 => "elf".to_string(),
        3 => "dwarf".to_string(),
        4 => "orc".to_string(),
        5 => "troll".to_string(),
        _ => "human".to_string()
    };
    */
    
    let playable_entity = spawn_referenced_entity(&RAWS.lock().unwrap(), world, &player_kind, player_starting_position);
    
    match playable_entity {
        None => { panic!("Can't create player.")},         
        Some(player_entity) => {
            // Surcharge du Nom.
            let mut entity_ref = world.entity_mut(player_entity);
            let mut name = entity_ref.get_mut::<Name>().unwrap();       // REMEMBER : C'est comme ca qu'on GET un component depuis WORLD.
            name.set("The Shadowrunner");

            // TODO : Pouvoir choisir.
            let mut rng = rand::thread_rng();
            let rand = rng.gen_range(1..4); // Exclusif 
            println!("rand is {:?}", rand);
            match rand {
                1 => { apply_referenced_job(&RAWS.lock().unwrap(), world, "adept", player_entity);},
                2 => { apply_referenced_job(&RAWS.lock().unwrap(), world, "gunslinger", player_entity);},
                3 => { apply_referenced_job(&RAWS.lock().unwrap(), world, "street_samourai", player_entity);},
                _ => { println!("No Job apply to PLAYER.");}
            }
 
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