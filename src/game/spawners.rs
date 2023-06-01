
// Spawners receive x,y positions WORLD based.
use bevy::prelude::*;

use crate::globals::SIZE_GHOUL;

use super::{player::{Player, Npc, Stats, Monster}, pieces::components::{Actor, Walk, Piece}};

pub fn spawn_player(
    commands: &mut Commands,
    size: i32
) -> Entity {
    let player = commands.spawn(Piece{size: size}).id();
    commands
        .entity(player)
        .insert(Player)
        .insert(Name::new("Player"))
        .insert(Stats {speed: 3.0})
        .insert(Actor::default(),)
        .id()  
}

pub fn spawn_npc(
    commands: &mut Commands,
) -> Entity {
    let npc = commands.spawn(Piece{size: SIZE_GHOUL}).id();
    commands
        .entity(npc)
        .insert(Name::new(format!("Ghoul")))
        .insert(Stats {speed: 2.0})
        .insert(Actor::default(),)
        .insert(Npc)
        .insert(Monster)
        .insert(Walk)
        .id()  
}



pub fn spawn_sprite_render(
    commands: &mut Commands,
    asset_server: &AssetServer,
    x: f32,
    y: f32,
    z: f32,
    img: &str,
) -> Entity {
    let sprite = commands.spawn(SpriteBundle {
        texture: asset_server.load(img),    //asset_server.load("temp_tiles/Sewers_wall.png"),
        transform: Transform {
            translation: Vec3::new(x, y, z),
            scale: Vec3::splat(1.0),   //splat(1.0),
            ..default()
        },
        ..default()
    }).id();

    sprite
}
