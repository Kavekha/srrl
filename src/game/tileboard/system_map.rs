use bevy::prelude::*;

use crate::{
    map_builders::map::Map, 
    states::GameState, 
    game::tileboard::components::{GridPosition, Tile, GameMap}
};


pub fn spawn_map(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut game_state: ResMut<NextState<GameState>>
) {
    println!("Map generation begins...");

    let mut tiles:Vec<Entity> = Vec::new();

    //We create logic entities from the map.tiles
    let mut x = 0;
    let mut y = 0;
    for (_idx, tile_info) in map.tiles.iter().enumerate(){
        let tile = commands.spawn((
            Tile {tiletype: *tile_info},
            GridPosition{x,y}
        ))
        .id();

        tiles.push(tile); 
          
        x += 1;
        if x > map.width as i32 - 1 {
            x = 0;
            y += 1;
        }
    }    

    commands
    .spawn(Name::new("Game Map"))
    .insert(GameMap)
    .push_children(&tiles)
    ;

    map.entity_tiles = tiles; 

    println!("Map generated.");

    game_state.set(GameState::GameMap); //TODO : Pas a ce systeme de gerer les changements de state.
}
