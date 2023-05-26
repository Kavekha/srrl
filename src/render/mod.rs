use bevy::prelude::*;

pub mod tilemap_render;
pub mod pieces;
pub mod components;

use self::{
    tilemap_render::spawn_map_render,
    pieces::spawn_piece_renderer,
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
            ;
    }
}


fn get_world_position(
    x: i32, 
    y: i32
) -> (f32, f32) {
        (TILE_SIZE * x as f32,
        0.0 - (TILE_SIZE * y as f32))   // La Grid part de haut vers le bas. La World Map monte en Y quand elle va vers le bas.
}