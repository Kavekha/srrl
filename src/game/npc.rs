use bevy::{
    prelude::*,
    sprite::collide_aabb::collide
};

use crate::{
    ascii::{
        spawn_ascii_sprite,
        AsciiSheet
    },
    TILE_SIZE, GameState, despawn_screen,
    game::{Player, Stats, TileCollider, TileExit},
};



pub struct NpcPlugin;


impl Plugin for NpcPlugin{
    fn build(&self, app: &mut App) {
        app         
            .add_systems(Update, npc_movement.run_if(in_state(GameState::GameMap)));         
    }
}


#[derive(Component)]
pub struct Npc;


pub fn spawn_npc(
    mut commands: &mut Commands, 
    ascii: &AsciiSheet
) {
    let npc = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        2,
        Color::rgb(0.3, 0.9, 0.4),
        Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
        Vec3::splat(1.0)
    );

    commands 
        .entity(npc)
        .insert(Npc)
        .insert(Name::new("Npc"))
        .insert(Stats {speed: 6.0});
}

fn npc_movement(){}