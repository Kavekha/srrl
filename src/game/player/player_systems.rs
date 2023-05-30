use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{
    save_load_system::ShouldSave, 
    commons::tile_collision_check,
    render::components::{TileExit}, 
    states::GameState, 
    game::{GridPosition, actions::{ActorQueue, models::WalkAction}, pieces::components::Actor}, 
    globals::{MULTI_DIR_KEY_MAPPING, MULTI_DIR_KEY_MAPPING_NO_NUM},
    map_builders::pathfinding::Position};


use super::{components::{Player, Stats}, PlayerInputReadyEvent};


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
        return;
    }
    
    // DEPLACEMENT
    //if keys.any_pressed([KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right]){
    if keys.any_pressed([
        KeyCode::Numpad1, KeyCode::Numpad2, KeyCode::Numpad3, KeyCode::Numpad4, KeyCode::Numpad6, KeyCode::Numpad7, KeyCode::Numpad8, KeyCode::Numpad9,
        KeyCode::Z, KeyCode::Q, KeyCode::S, KeyCode::D, KeyCode::A, KeyCode::E, KeyCode::W, KeyCode::X,
        ]){
        // On check si ca appartient à DIR_KEY_MAPPING et si rien du tout, on se barre car on veut pas envoyer d'event.
        let Ok((entity, entity_position, mut actor)) = query_player_position.get_single_mut() else {return};

        let mut destination = Position(0, 0);
        for (key, dir_position) in MULTI_DIR_KEY_MAPPING {
            if keys.pressed(key) {

                destination = Position(destination.0 + dir_position.0, destination.1 + dir_position.1);
            }
        }        
        for (key, dir_position) in MULTI_DIR_KEY_MAPPING_NO_NUM {
            if keys.pressed(key) {

                destination = Position(destination.0 + dir_position.0, destination.1 + dir_position.1);
            }
        }

        let final_destination = Position(entity_position.x + destination.0, entity_position.y + destination.1);
        let action = WalkAction(entity, final_destination);
        actor.0 = vec![(Box::new(action), 0)];      // 0 => Player doesn't care for Action Score.
        queue.0 = VecDeque::from([entity]);
        ev_input.send(PlayerInputReadyEvent);
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
