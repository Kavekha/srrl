use bevy::prelude::*;

use crate::{game::{game_generation::spawners::{create_npc, create_player}, pieces::spawners::{create_exit_map, create_nodes}, tileboard::system_map::create_map}, raws::{load_raws, spawn_named_kind, RAWS}};


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

    for spawn_position in map_infos.spawn_list {
        create_npc(world, spawn_position);
    }
    
    return true
}

