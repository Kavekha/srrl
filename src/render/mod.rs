use bevy::prelude::*;

pub mod tilemap_render;
pub mod pieces_render;
pub mod components;


use self::{
    tilemap_render::{spawn_map_render},
    pieces_render::{spawn_piece_renderer, update_piece_position},
};

use crate::{
    globals::{TILE_WIDTH_HALF, TILE_HEIGHT_HALF, TILE_HEIGHT_MEDIUM, }, 
    states::GameState, game::tileboard::components::BoardPosition,
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

pub fn get_world_position(
    position: &BoardPosition
) -> (f32, f32) {
        // REMEMBER : Y in bevy2d = Negative when going down!
        let iso_x = (position.v.x - position.v.y) * TILE_WIDTH_HALF;
        let iso_y = (position.v.x + position.v.y) * TILE_HEIGHT_HALF;
        
        (iso_x as f32,
        0.0 - iso_y as f32)     // REMEMBER : Y in bevy2d = Negative when going down!
}

/// z doit être calculé pour les objets à relief du genre mur. Le floor doit rester à 0 par contre.
fn get_world_z(
    position: &BoardPosition
) -> f32 {
    let z = (position.v.x as f32 / 10.0) + (position.v.y as f32 / 5.0);
    z
}



fn get_iso_y_modifier_from_elevation(
    tile_elevation: i32
) -> f32 {
    ((tile_elevation - TILE_HEIGHT_MEDIUM) / 2) as f32
}
