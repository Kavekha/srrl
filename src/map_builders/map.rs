use std::{
    fs::File,
    io::{BufReader, BufRead},
};

pub const MAP_HEIGHT: i32 = 50;
pub const MAP_WIDTH: i32 = 80;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Wall, 
    Floor,
    Exit
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    let map_width = MAP_WIDTH;
    (y as usize * map_width as usize) + x as usize      //TO CHANGE: we want to be able to choose height and width of the map
}

pub fn create_map_from_text() -> Vec<TileType>{
    let map_width = MAP_WIDTH;
    let map_height = MAP_HEIGHT;

    let mut map = vec![TileType::Floor; map_width as usize * map_height as usize];

    let file = File::open("assets/map.txt").expect("No map found");

    for (y, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line)= line {
            for (x, char) in line.chars().enumerate(){
                 if char == '#' {
                    map[xy_idx(x as i32, y as i32)] = TileType::Wall;
                }
                if char == '<' {
                    map[xy_idx(x as i32, y as i32)] = TileType::Exit;
                } else {
                    map[xy_idx(x as i32, y as i32)] = TileType::Floor;
                }
            }
        }
    }

    map 
}


pub fn new_map(
) -> Vec<TileType> {
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

    map
}