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