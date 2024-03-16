use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    map_builders::map::Map, 
    engine::states::GameState, 
    game::{tileboard::components::{Tile, GameMap, BoardPosition}, pieces::components::Occupier}, vectors::Vector2Int,
};


pub fn spawn_map(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut game_state: ResMut<NextState<GameState>>
) {
    println!("Map generation begins...");

    let mut tiles = HashMap::new();
    let mut tile_entities:Vec::<Entity> = Vec::new();

    //We create logic entities from the map.tiles
    let mut x = 0;
    let mut y = 0;
    for (_idx, tile_info) in map.tiles.iter().enumerate(){
        let v = Vector2Int::new(x, y);
        let tile = commands.spawn((
            Tile {tiletype: *tile_info},
            BoardPosition{v}
        ))
        .id();

        if map.is_blocked(x, y) {
            commands.entity(tile).insert(Occupier); //TODO : Something else? Occupier is used by Pieces too.
        }
        tiles.insert(v, tile); 
        tile_entities.push(tile);
          
        x += 1;
        if x > map.width as i32 - 1 {
            x = 0;
            y += 1;
        }
    }    
    
    commands
    .spawn(Name::new("Game Map"))
    .insert(GameMap)
    .push_children(&tile_entities)
    ;

    map.entity_tiles = tiles; 

    println!("Map generated.");

    game_state.set(GameState::GameMap); //TODO : Pas a ce systeme de gerer les changements de state.
}
