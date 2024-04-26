use bevy::prelude::*;

use crate::{game::{pieces::spawners::{create_exit_map, create_nodes, create_player, spawn_npcs}, tileboard::system_map::create_map}, raws::{load_raws, spawn_named_kind, RAWS}};


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

    for spawn in map_infos.spawn_list {
        spawn_named_kind(&RAWS.lock().unwrap(), world, "kind_human", spawn);
    }
    

    return true
}

