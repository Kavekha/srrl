use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use::bevy::prelude::*;

use crate::{
    map_builders::commons::TileType,
    globals::{MAPCOUNT, MAPHEIGHT, MAPWIDTH}, vectors::Vector2Int
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
    /* TODO: Never tested, may not work at all.
    pub fn idx_xy(
        &self,
        idx: usize
    ) -> (i32, i32) {
        let y = (idx / self.width as usize) as i32;
        let x = (idx - (y as usize * self.width as usize)) as i32;
        (x, y)
    }*/
    pub fn is_blocked(
        &self,
        x: i32,
        y: i32
    ) -> bool {
        let idx = self.xy_idx(x, y);
        self.blocked[idx]   
    }
    pub fn out_of_bounds(
        &self,
        x: i32,
        y: i32
    ) -> bool {
        if x < 0 || x > self.width -1 || y < 0 || y > self.height -1 { return true; } else { return false; };
    }

    /// Default map.
    pub fn new() -> Map {
        println!("Je fais un map::new(). La première vient de l'insertion de la Resource.");
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
