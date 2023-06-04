use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use::bevy::prelude::*;

use crate::{
    map_builders::{    
        commons::TileType,
        pathfinding::{Position, Successor}
    },
    globals::{MAPCOUNT, MAPHEIGHT, MAPWIDTH, DEFAULT_COST_PATHFINDING}, vectors::Vector2Int
};



#[derive(Resource, Clone, Default, Deserialize, Serialize, Debug)]  
pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub blocked: Vec<bool>,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub entity_tiles: HashMap<Vector2Int, Entity>   
}

impl Map {
    /// From x, y return the position from a one-entry vector.
    pub fn xy_idx(
        &self,
        x: i32, 
        y: i32
    ) -> usize {
        (y as usize * self.width as usize) + x as usize
    }
    pub fn is_blocked(
        &self,
        x: i32,
        y: i32
    ) -> bool {
        let idx = self.xy_idx(x, y);
        self.blocked[idx]   
    }

    /// Default map.
    pub fn new() -> Map {
        Map{
            tiles: vec![TileType::Wall; MAPCOUNT],  
            width: MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
            blocked: vec![false; MAPCOUNT],
            entity_tiles: HashMap::new()
        }
    }   

    pub fn populate_blocked(&mut self) {
        for (i,tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;  //self.blocked[i] = le resultat de tile == TileType::Wall = true!
        }
    }
}
