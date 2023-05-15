use std::{
    cmp::{max, min},
};
use rand::prelude::*;

use super::Map;
use super::MapBuilder;

use crate::{
    map_builders::{
        rectangle::Rectangle,
        commons::{TileType, apply_room_to_map, apply_horizontal_tunnel, apply_vertical_tunnel},
        pathfinding::Position,
    },   
};


pub struct SimpleMapBuilder {}

impl MapBuilder for SimpleMapBuilder {
    fn build() -> (Map, Position) {
        let mut map = Map::new();
        let player_position = SimpleMapBuilder::rooms_and_corridors(&mut map);
        (map, player_position)
    }
}

impl SimpleMapBuilder {
    pub fn rooms_and_corridors(map: &mut Map) -> Position {
        const MAX_ROOMS : i32 = 30;
        const MIN_SIZE : i32 = 6;
        const MAX_SIZE : i32 = 10;

        let mut rng = rand::thread_rng();

        for _ in 0..MAX_ROOMS {
            // generate a room as a Rectangle
            let w = rng.gen_range(MIN_SIZE.. MAX_SIZE);
            let h = rng.gen_range(MIN_SIZE.. MAX_SIZE);

            let x = rng.gen_range(1.. (map.width - w - 1)) - 1; 
            let y = rng.gen_range(1.. (map.height - h - 1)) - 1;

            let new_room = Rectangle::new(x, y, w, h);        

            // Can I add the room without intersecting with another?
            let mut can_add_room = true;

            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) { can_add_room = false }
            }
            if can_add_room {
                apply_room_to_map(map, &new_room);   

                // Join the new room to the previous one
                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len()-1].center();
                    if rng.gen_range(0.. 2) == 1 {
                        apply_horizontal_tunnel(map, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(map, prev_y, new_y, new_x);
                    } else {
                        apply_vertical_tunnel(map, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(map, prev_x, new_x, new_y);
                    }
                }     
                map.rooms.push(new_room);            
            }      
        }
        // Add an exit to the last room.
        let exit_position = map.rooms[map.rooms.len()-1].center();
        let exit_idx = map.xy_idx(exit_position.0, exit_position.1);
        map.tiles[exit_idx] = TileType::Exit;

        let start_pos = map.rooms[0].center();
        Position(start_pos.0, start_pos.1)  //RETURN
    }
}