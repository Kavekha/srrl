pub mod map;
pub mod commons;
pub mod rectangle;

pub const MAP_HEIGHT: i32 = 50;
pub const MAP_WIDTH: i32 = 80;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Wall, 
    Floor,
    Exit
}