use bevy::{prelude::*};

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    TILE_SIZE, despawn_screen, GameState, map_builders::map::MAP_WIDTH, map_builders::map::TileType,
};




#[derive(Component)]
pub struct TileCollider;

#[derive(Component)]
pub struct TileExit;

#[derive(Component)]
pub struct Map;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App){
        app
            .add_systems(OnEnter(GameState::GameMap), create_simple_map)
            .add_systems(OnExit(GameState::GameMap), despawn_screen::<Map>);     
    }
}

fn create_simple_map (
    mut commands: Commands, 
    ascii:Res<AsciiSheet>
) {

    let map_width = MAP_WIDTH;

    //we get map (vecTile) from a text file.
    let map = crate::map_builders::map::create_map_from_text();

    //All tiles created will go there
    let mut tiles:Vec<Entity> = Vec::new();

    //We create entities from this map.
    let mut x = 0;
    let mut y = 0;
    for (_idx, tile_info) in map.iter().enumerate(){

        //tiles.push(tile);     // Obligé de le faire dans chaque match car hors du scope :-()

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
        if x > map_width as i32 - 1 {
            x = 0;
            y += 1;
        }
    }    
    commands
        .spawn(Name::new("Map"))
        .insert(Map)
        .insert(SpatialBundle{
            ..default()
        })
        .push_children(&tiles);

}


    /*
    for (y, line) in map.iter() {
       for (x, tile) in line.iter(){
        if tile == crate::map_builders::map::TileType::Wall {
            let tile = spawn_ascii_sprite(
                &mut commands, 
                &ascii, 
                '#' as usize,
                Color::rgb(0.9, 0.9, 0.9),
                Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                Vec3::splat(1.0)
            );
            commands.entity(tile).insert(TileCollider);
        }
        if tile = TileType::Exit {
            let tile = spawn_ascii_sprite(
                &mut commands, 
                &ascii, 
                '<' as usize,
                Color::rgb(0.9, 0.9, 0.9),
                Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                Vec3::splat(1.0)
            );
            commands.entity(tile).insert(TileExit);
        } else {
            let tile = spawn_ascii_sprite(
                &mut commands, 
                &ascii, 
                '.' as usize,
                Color::rgb(0.9, 0.9, 0.9),
                Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                Vec3::splat(1.0)
            );
        }
        tiles.push(tile);
       }
    }
    commands
    .spawn(Name::new("Map"))
    .insert(Map)
    .insert(SpatialBundle{
        ..default()
    })
    .push_children(&tiles);
}
*/

/*
fn create_map_from_text (mut commands: Commands, ascii:Res<AsciiSheet>){
    let file = File::open("assets/map.txt").expect("No map found");
    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate(){
        if let Ok(line)= line {
            for (x, char) in line.chars().enumerate(){
                let tile = spawn_ascii_sprite(
                    &mut commands, 
                    &ascii, 
                    char as usize,
                    Color::rgb(0.9, 0.9, 0.9),
                    Vec3::new(x as f32 * TILE_SIZE, -(y as f32) * TILE_SIZE, 100.0),
                    Vec3::splat(1.0)
                );
                if char == '#' {
                    commands.entity(tile).insert(TileCollider);
                }
                if char == '<' {
                    commands.entity(tile).insert(TileExit);
                }
                tiles.push(tile);
            }
        }
    }

    commands
        .spawn(Name::new("Map"))
        .insert(Map)
        .insert(SpatialBundle{
            ..default()
        })
        .push_children(&tiles);

} */