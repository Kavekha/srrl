use std::{
    fs::File,
    io::{BufReader, BufRead},
};

use bevy::{prelude::*};

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    TILE_SIZE, despawn_screen, GameState,
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

fn create_simple_map (mut commands: Commands, ascii:Res<AsciiSheet>){
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

}