use bevy::prelude::*;

use crate::{game::{player::{Cursor, Player}, SPEED_MULTIPLIER}, globals::CHAR_SIZE};

pub const BASE_SPEED_CAMERA_SMOOTH_FOLLOW: f32 = 1.0;
pub const BORDER_TOLERANCE: f32 = CHAR_SIZE * 14.0;     // A partir de cette distance, on est considéré comme "dans le coin" de l'ecran et on peut bouger la camera.
pub const BORDER_NOT_TOLERATED: f32 = CHAR_SIZE * 3.0;  // A partir d'ici, on arrive dans le contour de l'ecran et c'est chiant que la camera se deplace.


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
pub fn _is_in_screen(
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

    if screen_position.x > (screen_size.x - BORDER_TOLERANCE) && screen_position.x < (screen_size.x - BORDER_NOT_TOLERATED) {
        camera_transform.translation.x = camera_transform.translation.x.lerp(
            res_cursor.world_position.x * 0.1 + camera_transform.translation.x + 0.9, 
            BASE_SPEED_CAMERA_SMOOTH_FOLLOW * SPEED_MULTIPLIER * time.delta_seconds())
    }
    if screen_position.x < (0.0 + BORDER_TOLERANCE) && screen_position.x > (0.0 + BORDER_NOT_TOLERATED) {
        camera_transform.translation.x = camera_transform.translation.x.lerp(
            0.0 + camera_transform.translation.x * 0.9, 
            BASE_SPEED_CAMERA_SMOOTH_FOLLOW * SPEED_MULTIPLIER * time.delta_seconds())
    }
    if screen_position.y > (screen_size.y - BORDER_TOLERANCE) && screen_position.y < (screen_size.y - BORDER_NOT_TOLERATED) {
        camera_transform.translation.y = camera_transform.translation.y.lerp(
            res_cursor.world_position.y * 0.1 + camera_transform.translation.y + 0.9, 
            BASE_SPEED_CAMERA_SMOOTH_FOLLOW * SPEED_MULTIPLIER * time.delta_seconds())
    }
    if screen_position.y < (0.0 + BORDER_TOLERANCE) && screen_position.y > (0.0 + BORDER_NOT_TOLERATED) {
        camera_transform.translation.y = camera_transform.translation.y.lerp(
            0.0 + camera_transform.translation.y * 0.9, 
            BASE_SPEED_CAMERA_SMOOTH_FOLLOW * SPEED_MULTIPLIER * time.delta_seconds())
    }
}



pub fn _camera_smooth_follow_v1(
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