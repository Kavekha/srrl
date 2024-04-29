use bevy::prelude::*;

use crate::{game::tileboard::components::{BoardPosition, ExitMapTile}, vectors::Vector2Int};

pub fn create_exit_map(world: &mut World, exit_position: Vector2Int){
    let mut exit = world.spawn_empty();
    exit 
    .insert(Name::new(format!("Exit")))
    .insert(ExitMapTile)
    .insert(BoardPosition{ v:exit_position});
info!("Exit map created");
}
