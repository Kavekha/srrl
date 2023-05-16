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


pub struct SimpleMapBuilder {
    map: Map,
    starting_position: Position,
}

impl MapBuilder for SimpleMapBuilder {
    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()        
    }

    fn build_map(&mut self) {
        self.rooms_and_corridors();
    }
    
    fn spawn_entities(&mut self) {
        for room in self.map.rooms.iter().skip(1){
            //TODO : We give mobs to spawn. 
            // BUT!!!! We dont have world access or commands access if we dont go through a system, and Bevy doesn't accept to send commands to Trait... :sad:
        }
    }
}

impl SimpleMapBuilder {
    pub fn new() -> SimpleMapBuilder {
        SimpleMapBuilder {
            map: Map::new(),
            starting_position: Position(0,0),
          }
    }

    pub fn rooms_and_corridors(&mut self) {
        const MAX_ROOMS : i32 = 30;
        const MIN_SIZE : i32 = 6;
        const MAX_SIZE : i32 = 10;

        let mut rng = rand::thread_rng();

        for _ in 0..MAX_ROOMS {
            // generate a room as a Rectangle
            let w = rng.gen_range(MIN_SIZE.. MAX_SIZE);
            let h = rng.gen_range(MIN_SIZE.. MAX_SIZE);

            let x = rng.gen_range(1.. (self.map.width - w - 1)) - 1; 
            let y = rng.gen_range(1.. (self.map.height - h - 1)) - 1;

            let new_room = Rectangle::new(x, y, w, h);        

            // Can I add the room without intersecting with another?
            let mut can_add_room = true;

            for other_room in self.map.rooms.iter() {
                if new_room.intersect(other_room) { can_add_room = false }
            }
            if can_add_room {
                apply_room_to_map(&mut self.map, &new_room);   

                // Join the new room to the previous one
                if !self.map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = self.map.rooms[self.map.rooms.len()-1].center();
                    if rng.gen_range(0.. 2) == 1 {
                        apply_horizontal_tunnel(&mut self.map, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut self.map, prev_y, new_y, new_x);
                    } else {
                        apply_vertical_tunnel(&mut self.map, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(&mut self.map, prev_x, new_x, new_y);
                    }
                }     
                self.map.rooms.push(new_room);            
            }      
        }
        // Add an exit to the last room.
        let exit_position = self.map.rooms[self.map.rooms.len()-1].center();
        let exit_idx = self.map.xy_idx(exit_position.0, exit_position.1);
        self.map.tiles[exit_idx] = TileType::Exit;

        let start_pos = self.map.rooms[0].center();
        self.starting_position = Position(start_pos.0, start_pos.1);
    }
}