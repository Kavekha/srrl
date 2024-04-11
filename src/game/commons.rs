use bresenham::Bresenham;

use crate::{map_builders::map::Map, vectors::Vector2Int};



pub fn is_in_sight(
    board: &Map,
    origin: &Vector2Int,
    end: &Vector2Int,
    range: i32
) ->Result<bool, bool> {
    //println!("BRESENHAM ==== ");
    //println!("origin is {:?}, end is {:?}", origin, end);
    let mut step = 0;
    for (x, y) in Bresenham::new((origin.x.try_into().unwrap(), origin.y.try_into().unwrap()), (end.x.try_into().unwrap(), end.y.try_into().unwrap())) {
        //println!("{}, {}", x, y);
        if board.is_blocked(x as i32, y as i32) { //(i32::from(x), i32::from(y)) {
            //println!("View is blocked");
            return Err(false)
        }
        step += 1;
        if step >= range {
            //println!("Max range reached.");
            return Err(false)
        }
    }
    //println!("View is clear!");
    return Ok(true)
}