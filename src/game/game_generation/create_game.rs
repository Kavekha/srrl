use bevy::prelude::*;

use crate::{game::{
    game_generation::{character_creation::create_player::create_player, create_exit_map::create_exit_map, create_nodes::create_nodes, spawners::spawn_room},  
    tileboard::system_map::create_map}, 
    map_builders::{Rectangle, MAPHEIGHT, MAPWIDTH}, raws::load_raws};


// Return true après avoir réussi.    // Return OK TODO
pub fn create_new_game(
    world: &mut World
) -> bool {
    info!("==== creating new game ===");
    load_raws();

    let map_infos = create_map(world);

    create_player(world, map_infos.starting_position);

    //spawn_npcs(world, map_infos.spawn_list);
    create_exit_map(world, map_infos.exit_position);
    create_nodes(world, map_infos.rooms);

    let room = &Rectangle::new(0, 0, MAPWIDTH as i32, MAPHEIGHT as i32); // Rectangle::new(0,0, MAPWIDTH, MAPHEIGHT);
    spawn_room(world, room);
    
    return true
}

