use bevy::{
    prelude::*,
    sprite::collide_aabb::collide
};

use crate::globals::{TILE_SIZE};


pub fn tile_collision_check(
    target_pos: Vec3,
    some_translation: Vec3
) -> bool {
    let collision = collide(
        target_pos,
        Vec2::splat(TILE_SIZE * 0.9),   //On reduit la box de collision pour ne pas être au pixel pret
        some_translation,
        Vec2::splat(TILE_SIZE)
    );
    collision.is_some()
}

/// Distance between 2 positions.
/// https://en.wikipedia.org/wiki/Taxicab_geometry
pub fn manhattan_distance(
    x: i32,
    y: i32,
    other_x: i32,
    other_y: i32) -> i32 {
    (x - other_x).abs() + (y - other_y).abs()
}