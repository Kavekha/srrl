use bevy::prelude::*;

pub mod tilemap_render;
pub mod pieces;
pub mod components;


use self::{
    tilemap_render::{spawn_map_render},
    pieces::{spawn_piece_renderer, update_piece_position},
};

use crate::{
    globals::{TILE_SIZE, TILE_WIDTH_HALF, TILE_HEIGHT_HALF, TILE_WIDTH, TILE_HEIGHT}, states::GameState
};




pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GraphicsWaitEvent>()

            .add_systems(OnEnter(GameState::GameMap), spawn_map_render)           
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
        // REMEMBER : Y in bevy2d = Negative when going down!
        let iso_x = (x - y) * TILE_WIDTH_HALF;
        let iso_y = (x + y) * TILE_HEIGHT_HALF;
        
        (iso_x as f32,
        0.0 - iso_y as f32)     // REMEMBER : Y in bevy2d = Negative when going down!

 
}