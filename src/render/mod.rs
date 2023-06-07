use bevy::prelude::*;

pub mod tilemap_render;
pub mod pieces_render;
pub mod components;


use self::{
    tilemap_render::{spawn_map_render},
    pieces_render::{spawn_piece_renderer, path_animator_update, walk_animation, melee_animation},
};

use crate::{
    globals::{TILE_WIDTH_HALF, TILE_HEIGHT_HALF, TILE_HEIGHT_MEDIUM, }, 
    states::{GameState, TurnSet}, vectors::Vector2Int,
};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GraphicsWaitEvent>()

            .add_systems(OnEnter(GameState::GameMap), spawn_map_render)           
            .add_systems(OnEnter(GameState::GameMap), spawn_piece_renderer)
            //.add_systems(Update, update_piece_position.run_if(in_state(GameState::GameMap)))

            .add_systems(Update, (walk_animation, path_animator_update, melee_animation).in_set(TurnSet::Animation))
    
            //.add_systems(Update, path_animator_update.run_if(in_state(GameState::GameMap)))
            //.add_systems(Update, walk_animation.run_if(in_state(GameState::GameMap)))
            
            ;
    }
}

#[derive(Event)]
pub struct GraphicsWaitEvent;



pub fn get_world_position(
    v: &Vector2Int
) -> (f32, f32) {
        // REMEMBER : Y in bevy2d = Negative when going down!
        let iso_x = (v.x - v.y) * TILE_WIDTH_HALF;
        let iso_y = (v.x + v.y) * TILE_HEIGHT_HALF;
        
        (iso_x as f32,
        0.0 - iso_y as f32)     // REMEMBER : Y in bevy2d = Negative when going down!
}

/// z doit être calculé pour les objets à relief du genre mur. Le floor doit rester à 0 par contre.
fn get_world_z(
    position: &Vector2Int
) -> f32 {
    let z = (position.x as f32 / 10.0) + (position.y as f32 / 5.0);
    z
}



fn get_iso_y_modifier_from_elevation(
    tile_elevation: i32
) -> f32 {
    ((tile_elevation - TILE_HEIGHT_MEDIUM) / 2) as f32
}
