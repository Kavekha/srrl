
// Spawners receive x,y positions WORLD based.
use bevy::prelude::*;

use super::{player::{Player, Npc, Stats, Monster, Piece}, pieces::components::Actor};

pub fn spawn_player(
    commands: &mut Commands,
) -> Entity {
    let player = commands.spawn(Piece).id();
    commands
        .entity(player)
        .insert(Player)
        .insert(Name::new("Player"))
        .insert(Stats {speed: 6.0})
        .insert(Actor::default(),)
        .id()  
}

pub fn spawn_npc(
    commands: &mut Commands,
) -> Entity {
    let npc = commands.spawn(Piece).id();
    commands
        .entity(npc)
        .insert(Name::new(format!("Ghoul")))
        .insert(Stats {speed: 2.0})
        .insert(Npc)
        .insert(Monster)
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
/* 

pub fn spawn_player_render(
    mut commands: &mut Commands,
    asset_server: &AssetServer,
    x: f32,
    y: f32
) -> Entity {
    let player = spawn_sprite(
        &mut commands, 
        &asset_server, 
        x,
        y,
        900.0,
        "temp_tiles/Gentera.png"
    );
     commands
        .entity(player)
        .insert(Player)
        .insert(Name::new("Player"))
        //.insert(Save)
        .insert(Stats {speed: 6.0});   

    player
}

pub fn spawn_npc_render(
    mut commands: &mut Commands,
    asset_server: &AssetServer,
    x: f32,
    y: f32,
    name: String,
) -> Entity {
    let npc = spawn_sprite(
        &mut commands, 
        &asset_server, 
        x,
        y,
        900.0,
        "temp_tiles/Nosferatu.png"
    );
    commands
    .entity(npc)
    .insert(Npc)
    .insert(Name::new(name))
    .insert(Stats {speed: 3.0});
    
    npc
}
*/

