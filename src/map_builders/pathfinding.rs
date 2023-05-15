use crate::TILE_SIZE;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position(pub i32, pub i32);
impl Position {
    pub fn distance(&self, other: &Position) -> i32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as i32
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct Successor {
    pub position: Position,
    pub cost: i32,
}

pub fn world_to_grid_position(
    x: f32,
    y: f32 
) -> (i32, i32) {
    //value.abs()
    let x_index = x / TILE_SIZE as f32;
    let y_index = y / TILE_SIZE as f32;
    (x_index.abs() as i32, y_index.abs() as i32)
}

/*
pub fn grid_to_world_position(
    x: i32,
    y: i32
) -> (f32, f32) {
    (x as f32 * TILE_SIZE,
    -(y as f32) * TILE_SIZE)
}
*/
