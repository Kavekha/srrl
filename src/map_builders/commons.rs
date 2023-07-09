use std::cmp::{max, min};
use serde::{Deserialize, Serialize};

use crate::map_builders::{
    rectangle::Rectangle,
    map::Map,
};



#[derive(PartialEq, Copy, Clone, Debug, Deserialize, Serialize)]
pub enum TileType {
    Wall, 
    Floor,
    Exit
}

pub fn apply_horizontal_tunnel(
    map: &mut Map,
    x1:i32, 
    x2:i32, 
    y:i32
) {    
    for x in min(x1,x2) ..= max(x1,x2) {
        let idx = map.xy_idx(x, y);
        if idx > 0 && idx < map.width as usize * map.height as usize {
            map.tiles[idx as usize] = TileType::Floor;
        }
    }
}

pub fn apply_vertical_tunnel(
    map: &mut Map,
    y1:i32, 
    y2:i32, 
    x:i32
) {    
    for y in min(y1,y2) ..= max(y1,y2) {
        let idx = map.xy_idx(x, y);
        if idx > 0 && idx < map.width as usize * map.height as usize {
            map.tiles[idx as usize] = TileType::Floor;
        }
    }
}
