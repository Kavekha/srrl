use bevy::prelude::*;

use crate::{engine::render::components::GameCursorRender, game::{player::{Cursor, Player}, SPEED_MULTIPLIER}, globals::CHAR_SIZE};

pub const BASE_SPEED_CAMERA_SMOOTH_FOLLOW: f32 = 1.0;
pub const BORDER_TOLERANCE: f32 = CHAR_SIZE * 10.0;  


pub fn camera_center_on_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let Ok(player_transform) = player_query.get_single() else {return};
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
    info!("Camera centered on player.");
}

// TODO : do better...
pub fn is_in_screen(
    screen_size: Vec2,  //let (camera, camera_transform) = camera_q.single(); let Some(screen_size) = camera.logical_viewport_size()
    screen_position: Vec2,  //let Some(mut screen_position) = camera.world_to_viewport(camera_transform, transform.translation) 
) -> bool {
    if screen_position.x < 0.0 || screen_position.x > screen_size.x || screen_position.y < 0.0 || screen_position.y > screen_size.x {
        return false
    }
    return true
}

pub fn camera_smooth_follow(
    mut camera_q: Query<(&Camera, &GlobalTransform, &mut Transform)>, 
    time: Res<Time>,
    res_cursor: Res<Cursor>,
) {
    let (camera, camera_gtransform, mut camera_transform) = camera_q.single_mut(); 
    let Some(screen_size) = camera.logical_viewport_size() else { return };
    let Some(screen_position) = camera.world_to_viewport(camera_gtransform, res_cursor.world_position) else { return };

    println!("Screen size is {:?}. Cursor is at {:?}", screen_size, screen_position);

    if screen_position.x > (screen_size.x - BORDER_TOLERANCE) {
        camera_transform.translation.x = camera_transform.translation.x.lerp(
            res_cursor.world_position.x * 0.1 + camera_transform.translation.x + 0.9, 
            BASE_SPEED_CAMERA_SMOOTH_FOLLOW * SPEED_MULTIPLIER * time.delta_seconds())
    }
    if screen_position.x < (0.0 + BORDER_TOLERANCE) {
        camera_transform.translation.x = camera_transform.translation.x.lerp(
            0.0 + camera_transform.translation.x * 0.9, 
            BASE_SPEED_CAMERA_SMOOTH_FOLLOW * SPEED_MULTIPLIER * time.delta_seconds())
    }
    if screen_position.y > (screen_size.y - BORDER_TOLERANCE) {
        camera_transform.translation.y = camera_transform.translation.y.lerp(
            res_cursor.world_position.y * 0.1 + camera_transform.translation.y + 0.9, 
            BASE_SPEED_CAMERA_SMOOTH_FOLLOW * SPEED_MULTIPLIER * time.delta_seconds())
    }
    if screen_position.y < (0.0 + BORDER_TOLERANCE) {
        camera_transform.translation.y = camera_transform.translation.y.lerp(
            0.0 + camera_transform.translation.y * 0.9, 
            BASE_SPEED_CAMERA_SMOOTH_FOLLOW * SPEED_MULTIPLIER * time.delta_seconds())
    }
}



pub fn camera_smooth_follow_v1(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.get_single() else {return};
    let mut camera_transform = camera_query.single_mut();

    if player_transform.translation.x != camera_transform.translation.x {
        camera_transform.translation.x = camera_transform.translation.x.lerp(
            player_transform.translation.x, 
            BASE_SPEED_CAMERA_SMOOTH_FOLLOW * SPEED_MULTIPLIER * time.delta_seconds())
    }
    if player_transform.translation.y != camera_transform.translation.y {
        camera_transform.translation.y = camera_transform.translation.y.lerp(
            player_transform.translation.y, 
            BASE_SPEED_CAMERA_SMOOTH_FOLLOW * SPEED_MULTIPLIER * time.delta_seconds())
    }
}