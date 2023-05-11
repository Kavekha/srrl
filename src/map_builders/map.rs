use std::{
    cmp::{max, min},
    fs::File,
    io::{BufReader, BufRead},
};

use::rand::prelude::*;
use::bevy::prelude::*;

use super::TileType;
use crate::map_builders::rectangle::{Rectangle};


pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rectangle>,
    pub width: i32,
    pub height: i32
}

impl Map {
    /// From x, y return the position from a one-entry vector.
    pub fn xy_idx(
        &self,
        x: i32, 
        y: i32
    ) -> usize {
        (y as usize * self.width as usize) + x as usize      //TO CHANGE: we want to be able to choose height and width of the map
    }

    pub fn apply_room_to_map(
        &mut self, 
        room : &Rectangle
    ) {
        for y in room.y1 +1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    } 

    pub fn apply_horizontal_tunnel(
        &mut self,
        x1:i32, 
        x2:i32, 
        y:i32
    ) {    
        for x in min(x1,x2) ..= max(x1,x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }
    
    pub fn apply_vertical_tunnel(
        &mut self,
        y1:i32, 
        y2:i32, 
        x:i32
    ) {    
        for y in min(y1,y2) ..= max(y1,y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map{
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: Vec::new(),      //Vec<Rectangle> = Vec::new();
            width: 80,
            height: 50
        };

        const MAX_ROOMS : i32 = 30;
        const MIN_SIZE : i32 = 6;
        const MAX_SIZE : i32 = 10;

        let mut rng = rand::thread_rng();

        //let mut last_room_center_idx: usize = 0;  //We want to know center of the last room to put an exit tile on it.
    
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
                map.apply_room_to_map(&new_room);   

                // Join the new room to the previous one
                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len()-1].center();
                    if rng.gen_range(0.. 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel( prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel( prev_x, new_x, new_y);
                    }
                }     
                map.rooms.push(new_room);            
            }      
        }
        // Add an exit to the last room. TODO : Player can be placed near the exit this way, but hey.
        let exit_position = map.rooms[map.rooms.len()-1].center();
        let exit_idx = map.xy_idx(exit_position.0, exit_position.1);
        map.tiles[exit_idx] = TileType::Exit;

        map
    }

    #[warn(dead_code)]
    pub fn new_map_from_textfile(
        file_name: &str
    ) -> Map {
        let mut map = Map{
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: Vec::new(),      //Vec<Rectangle> = Vec::new();
            width: 80,
            height: 50
        };

        let path = format!("assets/{}", file_name);
        let file = File::open(path).expect("No map found");

        for (y, line) in BufReader::new(file).lines().enumerate(){
            if let Ok(line)= line {
                for (x, character) in line.chars().enumerate(){
                    let idx = map.xy_idx(x as i32, y as i32);
                    match character {
                        // TOREMEMBER : fun story : "#" is a &str, but '#' is a char.
                        '<' => { map.tiles[idx] = TileType::Exit; }
                        '#' => { map.tiles[idx] = TileType::Wall; }
                        '.' => { map.tiles[idx] = TileType::Floor;}
                        _   => { map.tiles[idx] = TileType::Wall; }
                    }
                }
            }
        }
        map
    }
}
