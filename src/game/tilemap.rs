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
            .add_systems(Update, display_map_generation.run_if(in_state(GameState::MapGeneration)))     //SHOW_MAPGEN_VISUALIZER must be true.
            .add_systems(Update, despawn_screen::<GameMap>.run_if(in_state(GameState::MapGeneration)))     //SHOW_MAPGEN_VISUALIZER must be true.
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
    mut map_gen: ResMut<MapGenHistory>,
    time: Res<Time>,
){
    if !SHOW_MAPGEN_VISUALIZER{
        game_state.set(GameState::GameMap);
    }
    let map_generated = map_gen.history[map_gen.index].clone();
    println!("Current Snapshot from map history: {}", map_gen.index);
    generate_gamemap(&mut commands, &ascii, &map_generated);
    let mut timer = 0.0;

    // TODO : REFACTO: Fige le jeu pendant ce temps.
    while timer < 30.0 {
        let tick_time = 0.1 * time.delta_seconds();
        timer += tick_time;
        println!("Mon TIMER est de {:?} et le tick est de {:?}", timer, tick_time);
    }
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
