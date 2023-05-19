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
    game::{GameState, GameMap, TileCollider, TileExit},
    SHOW_MAPGEN_VISUALIZER
};



pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App){
        app
            .add_systems(OnEnter(GameState::MapGeneration), display_map_generation)     //SHOW_MAPGEN_VISUALIZER must be true.
            .add_systems(OnEnter(GameState::GameMap), create_map)
            .add_systems(OnExit(GameState::GameMap), despawn_screen::<GameMap>);     
    }
}

fn create_map(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    map: Res<Map>
) {
    generate_gamemap(&mut commands, &ascii, &map);
}


fn display_map_generation(
    mut game_state: ResMut<NextState<GameState>>,
    mut commands: Commands, 
    ascii:Res<AsciiSheet>,
    map_history: Res<MapGenHistory>,
    time: Res<Time>,
){
    if !SHOW_MAPGEN_VISUALIZER{
        game_state.set(GameState::GameMap);
    }

    //TIMER

    // ITERATE
    let mut index = 0;
    let mut game_map = generate_gamemap(&mut commands, &ascii, &map_history.history[index]);
    let mut timer = 0.0;

    while index < map_history.history.len() -1 {
        while timer < 300.0 {
            timer += time.delta_seconds();
            println!("Mon TIMER est de {:?}", timer);
        }
        println!("Next Snapshot from map history:");
        commands.entity(game_map).despawn_recursive();
        timer = 0.0;
        index += 1;
        game_map = generate_gamemap(&mut commands, &ascii, &map_history.history[index]);
    }

    // End of map generation history:
    println!("Fin de l'affichage de la generation history");
    game_state.set(GameState::GameMap);
}



pub fn generate_gamemap (
    mut commands: &mut Commands, 
    ascii:&AsciiSheet,
    map: &Map
) -> Entity {   
    println!("Map generation begins..");
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
        .push_children(&tiles)
        .id()

}
