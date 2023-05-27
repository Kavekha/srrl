use bevy::prelude::*;

pub mod tilemap_render;
pub mod pieces;
pub mod components;

use self::{
    tilemap_render::spawn_map_render,
    pieces::{spawn_piece_renderer, update_piece_position},
};

use crate::{
    globals::TILE_SIZE, states::GameState
};




pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::GameMap), spawn_map_render)
            .add_systems(OnEnter(GameState::GameMap), spawn_piece_renderer)
            .add_systems(Update, update_piece_position.run_if(in_state(GameState::GameMap)))
            ;
    }
}


fn get_world_position(
    x: i32, 
    y: i32
) -> (f32, f32) {
        (TILE_SIZE * x as f32,
        TILE_SIZE * y as f32)   
}