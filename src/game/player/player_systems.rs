use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{save_load_system::ShouldSave, commons::tile_collision_check, render::components::{TileExit}, states::GameState, game::{GridPosition, actions::{ActorQueue, models::WalkAction}, pieces::components::Actor}, map_builders::pathfinding::Position};

use super::{components::{Player, Stats}, PlayerInputReadyEvent};


//TODO : Check deplacement : si blocked 
pub fn player_input(
    mut query_player_position: Query<(Entity, &GridPosition, &mut Actor), With<Player>>,
    keys: Res<Input<KeyCode>>,
    mut should_save: ResMut<ShouldSave>,
    mut queue: ResMut<ActorQueue>,
    mut ev_input: EventWriter<PlayerInputReadyEvent>
){
    // MENU etc
    if keys.just_pressed(KeyCode::Escape) {
        should_save.to_save = true;
    }
    
    // DEPLACEMENT
    let Ok((entity, _position, mut actor)) = query_player_position.get_single_mut() else {return};
    let mut x = 0;
    let mut y = 0;
    
    if keys.any_pressed([KeyCode::Up, KeyCode::Z]) {
        y += 1;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        y -= 1;
    }

    if keys.any_pressed([KeyCode::Right, KeyCode::D]){
        x += 1;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::Q]){
        x -= 1;
    }
    let destination = Position(x, y);

    let action = WalkAction(entity, destination);
        actor.0 = Some(Box::new(action));
        queue.0 = VecDeque::from([entity]);
        ev_input.send(PlayerInputReadyEvent);
  
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
