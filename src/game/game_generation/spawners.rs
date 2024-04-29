
use bevy::{ecs::world::World, utils::HashMap};
use rand::Rng;

use crate::{globals::{SPAWN_MAX_ELEMENTS, SPAWN_SUBSTRACT_ELEMENT}, map_builders::{map::Map, Rectangle, TileType}, raws::{get_spawn_table, RAWS}, vectors::Vector2Int};

use super::{create_npcs::create_npc, random_table::RandomTable};




// TODO : C'est ici qu'on pourra peupler plus specifiquement une map.
fn get_room_table(key: &str) -> RandomTable {
    get_spawn_table(&RAWS.lock().unwrap(), key)
}



#[allow(clippy::map_entry)]
pub fn spawn_room(world: &mut World, room : &Rectangle) {
    let mut possible_targets = Vec::new();
    let has_map = world.get_resource::<Map>();
    if let Some(map) = has_map {
        for y in room.y1 + 1 .. room.y2 {
            for x in room.x1 + 1 .. room.x2 {
                let idx = map.xy_idx(x, y);
                if map.tiles[idx] == TileType::Floor {
                    possible_targets.push(Vector2Int { x:x, y : y});
                }
            }
        }
        spawn_region(world, &possible_targets);
    } 
}

pub fn spawn_region(world: &mut World, area : &[Vector2Int]) {
    let spawn_table = get_room_table("spawn_ghoul");        // TODO : C'est Hardcoded. Devrait être changé avec les Missions.
    let mut spawn_points : HashMap<Vector2Int, String> = HashMap::new();
    let mut areas : Vec<Vector2Int> = Vec::from(area);

    // Scope to keep the borrow checker happy
    {
        let mut rng = rand::thread_rng();
        let num_spawns = i32::min(areas.len() as i32, rng.gen_range(1..SPAWN_MAX_ELEMENTS) - SPAWN_SUBSTRACT_ELEMENT);  
        if num_spawns == 0 { return; }

        for _i in 0 .. num_spawns {
            let array_index = if areas.len() == 1 { 0usize } else { (rng.gen_range(1..areas.len() as i32)-1) as usize };
            let position = areas[array_index];
            spawn_points.insert(position, spawn_table.roll());
            areas.remove(array_index);
        }
    }

    // Actually spawn the monsters
    for spawn in spawn_points.iter() {
        spawn_entity(world, &spawn);
    }
}


fn spawn_entity(
    world: &mut World, 
    spawn : &(&Vector2Int, &String),

) {
    let npc_result = create_npc(world, spawn.1, *spawn.0);
    if npc_result.is_some() {
        return;
    }
} 