use bevy::prelude::*;

use crate::{globals::STANDARD_TILE_SIZE, vectors::Vector2Int};

pub fn get_world_position(
    v: &Vector2Int
) -> (f32, f32) {
        // REMEMBER : Y in bevy2d = Negative when going down!
        let x = v.x * STANDARD_TILE_SIZE;
        let y = v.y  * STANDARD_TILE_SIZE;
        //println!("GetWorldPosition : {:?} gives {:?}. World position get grid position : {:?}", (v.x, v.y), (iso_x, iso_y), get_grid_position(iso_x as f32, 0.0 - iso_y as f32));
        (x as f32,
        0.0 - y as f32)     // REMEMBER : Y in bevy2d = Negative when going down!
}

pub fn despawn_component<T: Component>(
    to_despawn: Query<Entity, With<T>>, 
    commands: &mut Commands
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}