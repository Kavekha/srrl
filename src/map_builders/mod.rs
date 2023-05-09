pub mod map;
pub mod rectangle;


#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Wall, 
    Floor,
    Exit
}