
// Spawners receive x,y positions WORLD based.

use bevy::prelude::*;

use crate::{
    ascii::{
        spawn_ascii_sprite,
        AsciiSheet,
    },
    game::{Player, Stats, Npc},
    map_builders::{
        pathfinding::{Position, world_to_grid_position},
    }
};


pub fn spawn_player(
    mut commands: &mut Commands, 
    ascii: &AsciiSheet,
    x: f32,
    y: f32
) -> Entity {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::rgb(0.3, 0.3, 0.9),
        Vec3::new(x, y, 900.0), //(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
        Vec3::splat(1.0)
    );

    commands 
        .entity(player)
        .insert(Player)
        .insert(Name::new("Player"))
        .insert(Stats {speed: 6.0});

    player
}


pub fn spawn_npc(
    mut commands: &mut Commands, 
    ascii: &AsciiSheet,
    x: f32,
    y: f32,
    name: String,
    glyph: usize,
) -> Entity {
    let npc = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        glyph as usize,
        Color::rgb(0.3, 0.9, 0.4),
        Vec3::new(x, y, 900.0), //(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
        Vec3::splat(1.0)
    );

    let (home_x, home_y) = world_to_grid_position(x, y);
    commands 
        .entity(npc)
        .insert(Npc{home:Position(home_x, home_y)})
        .insert(Name::new(name))
        .insert(Stats {speed: 3.0});

    npc
}