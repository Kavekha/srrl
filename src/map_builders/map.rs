use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use::bevy::prelude::*;

use crate::{
    map_builders::{commons::TileType, MAPCOUNT, MAPHEIGHT, MAPWIDTH},
    //globals::{MAPCOUNT, MAPHEIGHT, MAPWIDTH}, 
    vectors::Vector2Int
};


// 0.20i : On ajoute revealed_tiles.
#[derive(Resource, Clone, Default, Deserialize, Serialize, Debug)]  
pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub blocked: Vec<bool>,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub entity_tiles: HashMap<Vector2Int, Entity>,
    pub revealed_tiles: Vec<bool>,  // 0.20n
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
    /* TOTEST: Never tested, may not work at all.   JAMAIS UTILISE pour le moment.
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
    pub fn is_revealed(
        &self,
        x: i32,
        y: i32
    ) -> bool {
        let idx = self.xy_idx(x, y);
        self.revealed_tiles[idx]   
    }
    pub fn out_of_bounds(
        &self,
        x: i32,
        y: i32
    ) -> bool {
        if x < 0 || x > self.width -1 || y < 0 || y > self.height -1 { return true; } else { return false; };
    }
    // 0.20n
    pub fn revealing_tile(
        &mut self,
        x: i32,
        y: i32
    ) {
        let idx = self.xy_idx(x, y);
        self.revealed_tiles[idx] = true;
    }

    /// Default map.
    pub fn new<S : ToString>(width: i32, height: i32, name: S) -> Map {
        println!("Je fais un map::new(). La première vient de l'insertion de la Resource.");
        let map_tile_count = (width*height) as usize;
        crate::spatial::set_size(map_tile_count);

        Map{
            tiles: vec![TileType::Wall; map_tile_count],  
            width: MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
            blocked: vec![false; map_tile_count],
            entity_tiles: HashMap::new(),
            revealed_tiles: vec![false; map_tile_count],
        }
    }   

    pub fn populate_blocked(&mut self) {
        crate::spatial::populate_blocked_from_map(self);    // 0.20n
        /*
        for (i,tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;  //self.blocked[i] = le resultat de tile == TileType::Wall = true!
        }
        */
    }

    pub fn clear_content_index(&mut self) {
        crate::spatial::clear();
    }
}
