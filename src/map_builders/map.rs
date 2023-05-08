#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall, 
    Floor
}

pub const MAP_HEIGHT: i32 = 50;
pub const MAP_WIDTH: i32 = 80;

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * MAP_WIDTH) + x as usize      //TO CHANGE: we want to be able to choose height and width of the map
}

pub fn new_map(
) -> Vec<TileType> {
    //TO CHANGE
    let map_height: i32 = MAP_HEIGHT;
    let map_width: i32 = MAP_WIDTH;

    //create a map full of floor tiles.
    let mut map = vec![TileType::Floor; map_width * map_height];

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