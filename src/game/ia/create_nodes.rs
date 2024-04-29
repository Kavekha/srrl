use bevy::prelude::*;

use crate::{game::tileboard::components::BoardPosition, vectors::Vector2Int};

use super::components::NavigationNode;



pub fn create_nodes(
    world: &mut World, 
    nodes_list: Option<Vec<Vector2Int>>,
 ) {
    match nodes_list {
        Some(nodes) => {
            for node in nodes {        
                let mut nod = world.spawn_empty();
                nod.insert(NavigationNode);
                nod.insert(BoardPosition { v: node });
            }
        },
        None => {},
    }
 }