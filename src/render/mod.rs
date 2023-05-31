use bevy::prelude::*;

pub mod tilemap_render;
pub mod pieces;
pub mod components;


use self::{
    tilemap_render::{spawn_map_render, spawn_map_iso_render},
    pieces::{spawn_piece_renderer, update_piece_position},
};

use crate::{
    globals::TILE_SIZE, states::GameState
};




pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GraphicsWaitEvent>()

            .add_systems(OnEnter(GameState::GameMap), spawn_map_render)
            //.add_systems(OnEnter(GameState::GameMap), spawn_map_iso_render)            
            .add_systems(OnEnter(GameState::GameMap), spawn_piece_renderer)
            .add_systems(Update, update_piece_position.run_if(in_state(GameState::GameMap)))
            ;
    }
}


pub struct GraphicsWaitEvent;

fn get_world_position(
    x: i32, 
    y: i32
) -> (f32, f32) {
        // grid version
        /* 
        (TILE_SIZE * x as f32,
        TILE_SIZE * y as f32)  
        */

        //iso version
        const TILE_WIDTH:i32 = 64;
        const TILE_WIDTH_HALF:i32 = 32;
        const TILE_HEIGHT:i32 = 32;
        const TILE_HEIGHT_HALF:i32 = 16;

        let iso_x = (x - y) * TILE_WIDTH_HALF;
        let iso_y = (x + y) * TILE_HEIGHT_HALF;

        //let iso_x = x - y;
        //let iso_y = (x + y) /2;
        (iso_x as f32,
        iso_y as f32)

 
}