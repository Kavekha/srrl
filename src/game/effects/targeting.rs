use bevy::prelude::*;

use crate::{game::tileboard::components::BoardPosition, vectors::Vector2Int};


pub fn entity_position(
    world: &mut World, 
    target: Entity
) -> Option<Vector2Int> {    
    let mut position_q = world.query::<&BoardPosition>();
    if let Ok(position) = position_q.get(world, target) {
       return Some(position.v);
    }
    None
}