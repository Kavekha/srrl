use bevy::prelude::*;

use crate::{save_load_system::ShouldSave, globals::TILE_SIZE, commons::tile_collision_check, render::components::{TileCollider, TileExit}, states::GameState};

use super::components::{Player, Stats};



pub fn player_input(
    mut should_save: ResMut<ShouldSave>,
    mut player_query: Query<(&Player, &mut Transform, &Stats)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let (_player, mut transform, stats) = player_query.single_mut();

    let mut y_delta: f32 = 0.0;

    //EScape menu in game
    if keys.just_pressed(KeyCode::Escape) {
        should_save.to_save = true;
      }

    if keys.any_pressed([KeyCode::Up, KeyCode::Z]) {
        y_delta += stats.speed * TILE_SIZE * time.delta_seconds(); 
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        y_delta -= stats.speed * TILE_SIZE * time.delta_seconds(); 
    }

    let mut x_delta: f32 = 0.0;
    if keys.any_pressed([KeyCode::Right, KeyCode::D]){
        x_delta += stats.speed * TILE_SIZE * time.delta_seconds(); 
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::Q]){
        x_delta -= stats.speed * TILE_SIZE * time.delta_seconds(); 
    }

    let target: Vec3 = transform.translation + Vec3::new(x_delta, 0.0, 0.0);

    if !wall_query
        .iter()
        .any(|&transform|tile_collision_check(target, transform.translation))
        {
            transform.translation = target;
        }
    let target: Vec3 = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    
    if !wall_query
        .iter()
        .any(|&transform|tile_collision_check(target, transform.translation))
        {
            transform.translation = target;
        }

}




pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;

}


pub fn player_step_check(
    player_query: Query<(&Player, &mut Transform)>,
    exit_query: Query<&Transform, (With<TileExit>, Without<Player>)>,
    mut game_state: ResMut<NextState<GameState>>
) {
    // If player on collision with an exit...
    let (_player, player_transform) = player_query.single();
    if exit_query
        .iter()
        .any(|&transform|tile_collision_check(player_transform.translation, transform.translation))
        {
            println!("Exit !");      
            game_state.set(GameState::VictoryScreen); 
        }
}
