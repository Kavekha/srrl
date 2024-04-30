use bevy::prelude::*;

use crate::{engine::render::components::GameCursorRender, game::{player::Player, BASE_SPEED_CAMERA_SMOOTH_FOLLOW, SPEED_MULTIPLIER}};


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