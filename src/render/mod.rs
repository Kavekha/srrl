use bevy::prelude::*;

pub mod tilemap_render;
pub mod pieces_render;
pub mod components;
pub mod cursor_render;


use self::{
    tilemap_render::{spawn_map_render},
    pieces_render::{spawn_piece_renderer, path_animator_update, walk_animation, melee_animation}, cursor_render::{spawn_game_cursor, update_game_cursor},
};

use crate::{
    globals::{TILE_WIDTH_HALF, TILE_HEIGHT_HALF, TILE_HEIGHT_MEDIUM, }, 
    states::{GameState, TurnSet, EngineState}, vectors::Vector2Int,
};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GraphicsWaitEvent>()

            .add_systems(OnEnter(GameState::GameMap), spawn_map_render)       
            .add_systems(OnEnter(GameState::GameMap), spawn_piece_renderer)
            .add_systems(OnEnter(GameState::GameMap), spawn_game_cursor)         

            .add_systems(Update, (walk_animation, path_animator_update, melee_animation).in_set(TurnSet::Animation))
            .add_systems(Update, update_game_cursor)         
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

        //println!("GetWorldPosition : {:?} gives {:?}. World position get grid position : {:?}", (v.x, v.y), (iso_x, iso_y), get_grid_position(iso_x as f32, 0.0 - iso_y as f32));

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

fn get_final_world_position(
    v: Vector2Int,
    size:i32
) -> Vec3 {
    let (w_x, mut w_y) = get_world_position(&v); 
    let w_z = get_world_z(&v);
    w_y += get_iso_y_modifier_from_elevation(size);

    return Vec3::new(w_x, w_y, w_z)
}