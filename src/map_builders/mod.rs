pub mod map;
pub mod rectangle;
pub mod pathfinding;


#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Wall, 
    Floor,
    Exit
}