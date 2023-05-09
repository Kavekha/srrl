use std::cmp::{max, min};

use super::{MAP_HEIGHT, MAP_WIDTH, TileType};
use crate::map_builders::rectangle::{Rectangle};


pub fn xy_idx(
    x: i32, 
    y: i32
) -> usize {
    let map_width = MAP_WIDTH;
    (y as usize * map_width as usize) + x as usize      //TO CHANGE: we want to be able to choose height and width of the map
}


pub fn apply_room_to_map(
    room : &Rectangle, 
    map: &mut [TileType]
) {
    for y in room.y1 +1 ..= room.y2 {
        for x in room.x1 + 1 ..= room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
} 

pub fn apply_horizontal_tunnel(
    map: &mut [TileType], 
    x1:i32, 
    x2:i32, 
    y:i32
) {
    let map_height = MAP_HEIGHT;
    let map_width = MAP_WIDTH;

    for x in min(x1,x2) ..= max(x1,x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < map_width as usize * map_height as usize {
            map[idx as usize] = TileType::Floor;
        }
    }
}

pub fn apply_vertical_tunnel(
    map: &mut [TileType],
    y1:i32, 
    y2:i32, 
    x:i32
) {
    let map_height = MAP_HEIGHT;
    let map_width = MAP_WIDTH;

    for y in min(y1,y2) ..= max(y1,y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < map_width as usize * map_height as usize {
            map[idx as usize] = TileType::Floor;
        }
    }
}