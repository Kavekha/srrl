//TODO : Refacto conversion Map > Tilemap: Check this: https://github.com/frederickjjoubert/bevy-pathfinding/blob/main/src/tilemap.rs


use bevy::{prelude::*};

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    TILE_SIZE, despawn_screen,
    map_builders::{
        commons::TileType,
        map::{Map},
        MapGenHistory
    },
    game::{
        GameState,
        spawners::spawn_sprite,
    },
    ecs_elements::components::{ GameMap, TileCollider, TileExit,},
    SHOW_MAPGEN_VISUALIZER
};

const FIXED_MAPGEN_NEW_SNAPSHOT: f32 = 10.0;    // Doesn't look like 1 update / 10 secs, but let's go with it for now.
const MAP_WALL: &str = "temp_tiles/Sewers_wall.png";
const MAP_FLOOR: &str = "temp_tiles/Sewers_floor.png";




pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App){
        app
            //SHOW_MAPGEN_VISUALIZER must be true.
            .insert_resource(FixedTime::new_from_secs(FIXED_MAPGEN_NEW_SNAPSHOT))
            .add_systems(FixedUpdate, (
                display_map_generation, 
                despawn_screen::<GameMap>
            ).chain().run_if(
                in_state(GameState::MapGeneration)))           

            .add_systems(OnEnter(GameState::GameMap), create_map)
            .add_systems(OnExit(GameState::GameMap), despawn_screen::<GameMap>);     
    }
}

fn create_map(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    asset_server: Res<AssetServer>,
    map: Res<Map>
) {
    generate_gamemap(&mut commands, &ascii, &asset_server, &map);
}

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



pub fn generate_gamemap (
    mut commands: &mut Commands, 
    ascii:&AsciiSheet,
    asset_server: &AssetServer,
    map: &Map
) -> Entity {   
    println!("Map generation begins..");
    //All tiles entities created will go there
    let mut tiles:Vec<Entity> = Vec::new();

    //We create entities from the map.tiles
    let mut x = 0;
    let mut y = 0;
    for (_idx, tile_info) in map.tiles.iter().enumerate(){
        // Convert to World Units.
        let world_x = x as f32 * TILE_SIZE;
        let world_y = -(y as f32 * TILE_SIZE);
        match tile_info {
            TileType::Wall => {
                /*
                let tile = spawn_ascii_sprite(
                    &mut commands, 
                    &ascii, 
                    '#' as usize,
                    Color::rgb(0.9, 0.9, 0.9),
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                    Vec3::splat(1.0)
                );
                 */
                let tile = spawn_sprite(
                    &mut commands,
                    &asset_server,
                    world_x,
                    world_y,
                    0.0,
                    MAP_WALL,
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
                let tile = spawn_sprite(
                    &mut commands,
                    &asset_server,
                    world_x,
                    world_y,
                    0.0,
                    MAP_FLOOR,
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
        .push_children(&tiles)
        .id()

}
