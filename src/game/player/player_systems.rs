use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{
    save_load_system::ShouldSave, 
    commons::tile_collision_check,
    render::components::{TileExit}, 
    states::GameState, 
    game::{actions::{ActorQueue, MoveToAction, ClearPendingAction, WalkOrHitAction, CancelPlayerPendingActionsEvent}, pieces::components::{Actor}, tileboard::components::BoardPosition}, 
    vectors::Vector2Int};


use super::{components::{Player}, PlayerActionEvent, Cursor};

//
pub const MULTI_DIR_KEY_MAPPING: [(KeyCode, Vector2Int); 8] = [
    (KeyCode::Numpad8, Vector2Int::UP), (KeyCode::Numpad2, Vector2Int::DOWN),
    (KeyCode::Numpad4, Vector2Int::LEFT), (KeyCode::Numpad6, Vector2Int::RIGHT),
    (KeyCode::Numpad7, Vector2Int::UPPER_LEFT), (KeyCode::Numpad9, Vector2Int::UPPER_RIGHT),
    (KeyCode::Numpad1, Vector2Int::BOTTOM_LEFT), (KeyCode::Numpad3, Vector2Int::BOTTOM_RIGHT),  
];

pub const MULTI_DIR_KEY_MAPPING_NO_NUM: [(KeyCode, Vector2Int); 8] = [
    (KeyCode::Z, Vector2Int::UP), (KeyCode::S, Vector2Int::DOWN),
    (KeyCode::Q, Vector2Int::LEFT), (KeyCode::D, Vector2Int::RIGHT),
    (KeyCode::A, Vector2Int::UPPER_LEFT), (KeyCode::E, Vector2Int::UPPER_RIGHT),
    (KeyCode::W, Vector2Int::BOTTOM_LEFT), (KeyCode::X, Vector2Int::BOTTOM_RIGHT),  
];
//

/* 
pub fn clear_player_pending_actions_2(
    mut player_queue: ResMut<PlayerActions>, 
){
    player_queue.0.clear();
}*/

pub fn player_mouse_input(
    buttons: Res<Input<MouseButton>>,
    mut ev_action: EventWriter<PlayerActionEvent>,
    mut query_player_actor: Query<(Entity, &mut Actor), With<Player>>,
    res_cursor: Res<Cursor>,
    mut queue: ResMut<ActorQueue>,
    mut ev_cancel: EventWriter<CancelPlayerPendingActionsEvent>,    
    //mut player_queue: ResMut<PlayerActions>, 
){
    if buttons.just_pressed(MouseButton::Right) {
        ev_cancel.send(CancelPlayerPendingActionsEvent);
    }
    if buttons.just_pressed(MouseButton::Left) {
        println!("Mouse button press Left");
        // On annule les actions en attente:
        ev_cancel.send(CancelPlayerPendingActionsEvent);
        println!("Canceling current actions....");
    }
    if buttons.just_released(MouseButton::Left) {
        /*
        //TODO : Qq chose de plus generique, car trop de duplication de code pour ca.
        let Ok((entity, mut actor)) = query_player_actor.get_single_mut() else {return};
        let action = ClearPendingAction(entity);
        actor.0 = vec![(Box::new(action), 0)];      // 0 => Player doesn't care for Action Score.
        queue.0 = VecDeque::from([entity]);
        println!("Player pending actions cleared. Actor queue len is : {:?}", actor.0.len());
         */

        println!("Doing new actions instead...");
        // MoveTo.
        let Ok((entity, mut actor)) = query_player_actor.get_single_mut() else {return};
        let destination = res_cursor.grid_position;
        
        let action = MoveToAction(entity, destination);
        actor.0 = vec![(Box::new(action), 0)];      // 0 => Player doesn't care for Action Score.
        queue.0 = VecDeque::from([entity]);
        ev_action.send(PlayerActionEvent);
        println!("MoveToAction sent from player mouse input. Actor queue len is : {:?}", actor.0.len())
    }
}

pub fn player_input(
    mut query_player_position: Query<(Entity, &BoardPosition, &mut Actor), With<Player>>,
    keys: Res<Input<KeyCode>>,
    mut should_save: ResMut<ShouldSave>,
    mut queue: ResMut<ActorQueue>,
    //mut ev_input: EventWriter<PlayerInputReadyEvent>,
    mut ev_action: EventWriter<PlayerActionEvent>,  
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
        let Ok((entity, position, mut actor)) = query_player_position.get_single_mut() else {return};

        // On annule les actions en cours.
        //TODO : Qq chose de plus generique, car trop de duplication de code pour ca.
        let action = ClearPendingAction(entity);
        actor.0 = vec![(Box::new(action), 0)];      // 0 => Player doesn't care for Action Score.
        queue.0 = VecDeque::from([entity]);
        println!("Player pending actions cleared.");

        let mut destination = position.v;
        for (key, dir_position) in MULTI_DIR_KEY_MAPPING {
            if keys.pressed(key) {
                destination += dir_position;
            }
        }        
        for (key, dir_position) in MULTI_DIR_KEY_MAPPING_NO_NUM {
            if keys.pressed(key) {
                destination += dir_position;
            }
        }

        //let action = WalkAction(entity, destination);
        let action = WalkOrHitAction(entity, destination);
        actor.0 = vec![(Box::new(action), 0)];      // 0 => Player doesn't care for Action Score.
        queue.0 = VecDeque::from([entity]);
        ev_action.send(PlayerActionEvent);
        println!("Keyboard: WalkAction: PlayeractionEvent sent 2");
    }
}



pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>
) {
    let Ok(player_transform) = player_query.get_single() else {return};
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
    let Ok((_player, player_transform)) = player_query.get_single() else {return };
    if exit_query
        .iter()
        .any(|&transform|tile_collision_check(player_transform.translation, transform.translation))
        {
            println!("Exit !");      
            game_state.set(GameState::VictoryScreen); 
        }
}
