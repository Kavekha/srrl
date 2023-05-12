use bevy::{prelude::*};

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    TILE_SIZE, despawn_screen,
    map_builders::{
        TileType,
        map::{Map}
    },
    game::{GameState, GameMap, TileCollider, TileExit, Game}
};



pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App){
        app
            //.add_systems(OnEnter(GameState::GameMap), create_map_from_text)
            .add_systems(OnEnter(GameState::GameMap), create_simple_random_map)
            .add_systems(OnExit(GameState::GameMap), despawn_screen::<GameMap>);     
    }
}


/// Deprecated, equivalent dans Game pour l'Init.   //TODO Refacto
fn create_simple_random_map(
    commands: Commands,
    ascii: Res<AsciiSheet>,
    game: Res<Game>
) {
    let new_map = &game.map;
    create_gamemap(commands, ascii, &new_map);
}

#[warn(dead_code)]
fn create_map_from_text(
    commands: Commands,
    ascii: Res<AsciiSheet>
){
    //we get map (vecTile) from a text file.
    let map: Map = Map::new_map_from_textfile("map.txt");

    create_gamemap(commands, ascii, &map);
}

pub fn create_gamemap (
    mut commands: Commands, 
    ascii:Res<AsciiSheet>,
    map: &Map
) {   
    //All tiles entities created will go there
    let mut tiles:Vec<Entity> = Vec::new();

    //We create entities from the map.tiles
    let mut x = 0;
    let mut y = 0;
    for (_idx, tile_info) in map.tiles.iter().enumerate(){
        match tile_info {
            TileType::Wall => {
                let tile = spawn_ascii_sprite(
                    &mut commands, 
                    &ascii, 
                    '#' as usize,
                    Color::rgb(0.9, 0.9, 0.9),
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                    Vec3::splat(1.0)
                );
                commands.entity(tile).insert(TileCollider);
                tiles.push(tile); 
            }
            TileType::Exit => {
                let tile = spawn_ascii_sprite(
                    &mut commands, 
                    &ascii, 
                    '<' as usize,
                    Color::rgb(0.9, 0.9, 0.9),
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                    Vec3::splat(1.0)
                );
                commands.entity(tile).insert(TileExit);
                tiles.push(tile); 
            }
            TileType::Floor => {
                let tile = spawn_ascii_sprite(
                    &mut commands, 
                    &ascii, 
                    '.' as usize,
                    Color::rgb(0.9, 0.9, 0.9),
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                    Vec3::splat(1.0)
                );
                tiles.push(tile); 
            }
        }            
        x += 1;
        if x > map.width as i32 - 1 {
            x = 0;
            y += 1;
        }
    }    
    commands
        .spawn(Name::new("Game Map"))
        .insert(GameMap)
        .insert(SpatialBundle{
            ..default()
        })
        .push_children(&tiles);

}
