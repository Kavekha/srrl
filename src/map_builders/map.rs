/*
use std::{
    cmp::{max, min},
    fs::File,
    io::{BufReader, BufRead},
};
*/

use::bevy::prelude::*;

use crate::{
    map_builders::{    
        commons::TileType,
        rectangle::Rectangle,
        pathfinding::{Position, Successor}
    },
};


pub const MAPWIDTH : usize = 80;
pub const MAPHEIGHT : usize = 50;
pub const MAPCOUNT : usize = MAPHEIGHT * MAPWIDTH;


#[derive(Resource, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub blocked: Vec<bool>,
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
        }
    }   

    pub fn populate_blocked(&mut self) {
        for (i,tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;  //self.blocked[i] = le resultat de tile == TileType::Wall
        }
    }
    // TODO : generate map in Bevy.
    pub fn create_game_map(){}

    pub fn get_successors(
        &self, 
        position: &Position
     ) -> Vec<Successor> {
        let mut successors = Vec::new();
         for dy in -1..=1 {
            //println!("dy is {}", dy);
            for dx in -1..=1 {
                //println!("dx is {}", dx);
                let x = position.0 + dx;
                let y = position.1 + dy;
                //println!("x and y are: {},{}", x, y);
                if dx == 0 && dy == 0 {
                    //println!("dx & dy = 0, out");
                    continue;
                } // Exclude current position.
                if x < 0 || x > self.width - 1 {
                    //println!("width bound nok, out");
                    continue;
                } // Make sure we are within width bounds.
                if y < 0 || y > self.height - 1 {
                    //println!("Is y < 0 ? {} < 0", y);
                    //println!("Is y > self height? {} > {}", y, self.height - 1);
                    //println!("height bound nok, continue");
                    continue;
                } // Make sure we are within height bounds.
    
                //println!("All check OK");
    
                let neighbor_position = Position(x, y);
                //println!("neigbhor position : {},{}", x, y);
                let neighbor_index = self.xy_idx(x, y);
                //println!("neighbor_index is {}", neighbor_index);
                if self.blocked[neighbor_index] {
                    //println!("neighbor index is blocked, nok");
                    //println!("is_blocked should be True: {:?}", self.is_blocked(x, y)); // OK
                    continue;
                }
                //println!("Valid tile : should be false on is_blocked: {:?}", self.is_blocked(x, y));
                successors.push(Successor {
                    position: neighbor_position,
                    cost: 1,
                })
            }            
        }
        successors
    }

    /*
    #[warn(dead_code)]
    pub fn new_map_from_textfile(
        file_name: &str
    ) -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; 80 * 50],
            rooms: Vec::new(),      //Vec<Rectangle> = Vec::new();
            width: 80,
            height: 50,
            blocked: vec![false; 80 * 50],
        };

        let path = format!("assets/{}", file_name);
        let file = File::open(path).expect("No map found");

        for (y, line) in BufReader::new(file).lines().enumerate(){
            if let Ok(line)= line {
                for (x, character) in line.chars().enumerate(){
                    let idx = map.xy_idx(x as i32, y as i32);
                    match character {
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
    */
}
