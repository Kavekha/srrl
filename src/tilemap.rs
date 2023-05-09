use bevy::{prelude::*};

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    player::{Player},
    TILE_SIZE, despawn_screen, GameState, 
    map_builders::{
        TileType,
        map::{Map}
    },
};


#[derive(Component)]
pub struct TileCollider;

#[derive(Component)]
pub struct TileExit;

#[derive(Component)]
pub struct GameMap;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App){
        app
            //.add_systems(OnEnter(GameState::GameMap), create_map_from_text)
            .add_systems(OnEnter(GameState::GameMap), create_simple_random_map)
            .add_systems(OnExit(GameState::GameMap), despawn_screen::<GameMap>);     
    }
}

fn create_simple_random_map(
    commands: Commands,
    ascii: Res<AsciiSheet>,
    mut player_query: Query<&mut Transform, With<Player>>,
){
    let map: Map = Map::new_map_rooms_and_corridors();

    // Modify Player position.
    let mut player_transform = player_query.single_mut();       //TODO check si Player existe.
    let (x, y) = map.rooms[0].center();

    player_transform.translation.x = x as f32 * TILE_SIZE;
    player_transform.translation.y = -(y as f32) * TILE_SIZE;   //TODO : Pas relou déjà d'avoir du negatif qui se balade ici et là. OSKOUR.
    println!("player new position : {},{}", player_transform.translation.x, player_transform.translation.y); 
    
    create_gamemap(commands, ascii, map);
}

fn create_map_from_text(
    commands: Commands,
    ascii: Res<AsciiSheet>
){
    //we get map (vecTile) from a text file.
    let map: Map = Map::new_map_from_textfile("map.txt");

    create_gamemap(commands, ascii, map);
}

fn create_gamemap (
    mut commands: Commands, 
    ascii:Res<AsciiSheet>,
    map: Map
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
