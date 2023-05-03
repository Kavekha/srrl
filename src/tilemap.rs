use std::{
    fs::File,
    io::{BufReader, BufRead},
};

use bevy::{prelude::*};

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    TILE_SIZE,
};

pub struct TileMapPlugin;

#[derive(Component)]
pub struct TileCollider;



impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, create_simple_map);
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
                );
                if char == '#' {
                    commands.entity(tile).insert(TileCollider);
                }
                tiles.push(tile);
            }
        }
    }

    commands
        .spawn(Name::new("Map"))
        .insert(SpatialBundle{
            ..default()
        })
        //.insert(Transform::default())
        //.insert(GlobalTransform::default())
        .push_children(&tiles);

}