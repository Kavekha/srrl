use std::{
    fs::File,
    io::{BufReader, BufRead},
};

use bevy::prelude::system_adapter::new;
use::rand::prelude::*;

use super::{MAP_HEIGHT, MAP_WIDTH, TileType};
use crate::map_builders::rectangle::{Rectangle};
use crate::map_builders::commons::{
    xy_idx, apply_room_to_map, 
    apply_horizontal_tunnel, apply_vertical_tunnel 
};


pub fn new_map_rooms_and_corridors() -> (Vec<Rectangle>, Vec<TileType>) {
    let map_height = MAP_HEIGHT;
    let map_width = MAP_WIDTH;

    let mut map = vec![TileType::Wall; map_width as usize * map_height as usize];

    let mut rooms : Vec<Rectangle> = Vec::new();
    const MAX_ROOMS : i32 = 30;
    const MIN_SIZE : i32 = 6;
    const MAX_SIZE : i32 = 10;

    let mut rng = rand::thread_rng();

    let mut last_room_center_idx: usize = 0;  //We want to know center of the last room to put an exit tile on it.
 
    for _ in 0..MAX_ROOMS {
        // generate a room as a Rectangle
        let w = rng.gen_range(MIN_SIZE.. MAX_SIZE);
        let h = rng.gen_range(MIN_SIZE.. MAX_SIZE);

        let x = rng.gen_range(1.. (map_width - w - 1)) - 1; 
        let y = rng.gen_range(1.. (map_height - h - 1)) - 1;

        let new_room = Rectangle::new(x, y, w, h);        

        // Can I add the room without intersecting with another?
        let mut can_add_room = true;

        for other_room in rooms.iter() {
            if new_room.intersect(other_room) { can_add_room = false }
        }
        if can_add_room {
            let (center_x, center_y) = new_room.center().clone();
            last_room_center_idx = xy_idx(center_x as i32, center_y as i32);
            apply_room_to_map(&new_room, &mut map);   

            // Join the new room to the previous one
            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len()-1].center();
                if rng.gen_range(0.. 2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }     
            rooms.push(new_room);            
        }      
    }
    // Add an exit.
    map[last_room_center_idx] = TileType::Exit;


    (rooms, map)
}


pub fn create_simple_map() -> Vec<TileType> {
    //TO CHANGE
    let map_height: i32 = MAP_HEIGHT;
    let map_width: i32 = MAP_WIDTH;

    //create a map full of floor tiles.
    let mut map = vec![TileType::Floor; map_width as usize * map_height as usize];

    //Boundaries walls
    for x in 0..map_width {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, map_height -1)] = TileType::Wall;
    }
    for y in 0..map_height {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(map_width -1, y)] = TileType::Wall;      
    }

    //random walls
    let mut rng = rand::thread_rng();

    for _i in 0..400 {
        let x = rng.gen_range(1.. 79);
        let y = rng.gen_range(1.. 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }
    //add Exit tile
    map[xy_idx(map_height / 2, map_width /2)] = TileType::Exit;

    map
}


pub fn create_map_from_text() -> Vec<TileType> {
    let map_width = MAP_WIDTH;
    let map_height = MAP_HEIGHT;

    let mut map = vec![TileType::Floor; map_width as usize * map_height as usize];

    let file = File::open("assets/map.txt").expect("No map found");

    for (y, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line)= line {
            for (x, character) in line.chars().enumerate(){
                match character {
                    // TOREMEMBER : fun story : "#" is a &str, but '#' is a char.
                    '<' => { map[xy_idx(x as i32, y as i32)] = TileType::Exit; }
                    '#' => { map[xy_idx(x as i32, y as i32)] = TileType::Wall; }
                    _   => { map[xy_idx(x as i32, y as i32)] = TileType::Floor;}
                }
            }
        }
    }
    map
}
