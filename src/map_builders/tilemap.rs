//Here goes the TileMap logic.

use bevy::{prelude::*};

use crate::{
    ascii::{spawn_ascii_sprite, },
    globals::{TILE_SIZE, MAP_WALL, MAP_FLOOR},
    despawn_screen,
    map_builders::{
        commons::TileType,
        map::{Map},
    },
    game::{
        spawners::spawn_sprite,
    },
    ecs_elements::{
        components::{ GameMap, TileCollider, TileExit, GridPosition, Tile, GameMapRender},
        resources::{MapGenHistory,AsciiSheet,GameState},
    },
    globals::{SHOW_MAPGEN_VISUALIZER, FIXED_MAPGEN_NEW_SNAPSHOT},
};


pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App){
        app
            // Init.
            .add_systems(OnEnter(GameState::Prerun), spawn_map)
            
            //SHOW_MAPGEN_VISUALIZER must be true.  //TODO : Broken lors de la division Logic VS Render.
            /* 
            .insert_resource(FixedTime::new_from_secs(FIXED_MAPGEN_NEW_SNAPSHOT))
            .add_systems(FixedUpdate, (
                display_map_generation, 
                despawn_screen::<GameMap>
            ).chain().run_if(
                in_state(GameState::MapGeneration)))       
            */

            .add_systems(OnExit(GameState::GameMap), despawn_screen::<GameMap>)     
            .add_systems(OnExit(GameState::GameMap), despawn_screen::<GameMapRender>)
            ;  
    }
}

// Logic map.
fn spawn_map(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut game_state: ResMut<NextState<GameState>>
) {
    println!("Map generation begins..");

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

    game_state.set(GameState::GameMap); //TODO : Pas a ce systeme de gerer les changements de state.
}


/* 
/// TODO: Broken lors de la separation Logic vs Render: on n'utilise plus de fonction generate_gamemap.
fn display_map_generation(
    mut game_state: ResMut<NextState<GameState>>,
    mut commands: Commands, 
    ascii:Res<AsciiSheet>,
    mut map_gen: ResMut<MapGenHistory>,
    time: Res<Time>,
    last_time: Local<f32>,
    asset_server: Res<AssetServer>,
){
    println!(
        "time since last fixed_update: {}\n",
        time.raw_elapsed_seconds() - *last_time
    );

    if !SHOW_MAPGEN_VISUALIZER{
        game_state.set(GameState::GameMap);
    }
    let map_generated = map_gen.history[map_gen.index].clone();
    println!("Current Snapshot from map history: {}", map_gen.index);
    generate_gamemap(&mut commands, &ascii, &asset_server,&map_generated);
    map_gen.index += 1;
    

    // End of map generation history:
    if map_gen.index >= map_gen.history.len(){
        println!("Fin de l'affichage de la generation history");
        game_state.set(GameState::GameMap);
    }
}
*/
