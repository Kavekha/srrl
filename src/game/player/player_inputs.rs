use bevy::{prelude::*, input::mouse::MouseMotion};

use crate::game::{combat::{event_systems::ActionInfos, events::{EntityEndTurnEvent, EntityTryMoveEvent, RefreshActionCostEvent}}, manager::{change_state_messages::{ChangeGameStateRunningMessage, ChangeGameStateUnavailableMessage}, menu_messages::{CloseMenuMessage, OpenInGameMenuOpenMessage}, MessageEvent}};

use super::{components::OnClickEvent, Cursor, Player};


// TODO : Est-ce normal que le cout AP soit ici?
pub fn player_mouse_input(
    mut ev_refresh_action: EventWriter<RefreshActionCostEvent>,
    mut mouse_move: EventReader<MouseMotion>,
){
    for _event in mouse_move.read() {
        ev_refresh_action.send(RefreshActionCostEvent);
    }
}


// GameState is Running, I can call Menu.
pub fn ig_call_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_message: EventWriter<MessageEvent>  
){
    // MENU etc
    if keys.just_pressed(KeyCode::Escape) {
        println!("Call for In Game Menu.");        
        ev_message.send(MessageEvent(Box::new(OpenInGameMenuOpenMessage))); 
        ev_message.send(MessageEvent(Box::new(ChangeGameStateUnavailableMessage))); 
    }
}

// GameState is Unavailable, I can close the menu.
pub fn ig_inside_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_message: EventWriter<MessageEvent>  
){
    // MENU etc
    if keys.just_pressed(KeyCode::Escape) {
        println!("Back to game.");
        ev_message.send(MessageEvent(Box::new(ChangeGameStateRunningMessage))); 
        ev_message.send(MessageEvent(Box::new(CloseMenuMessage)));         
    }
}


/// Les events du Joueur.
pub fn combat_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_endturn: EventWriter<EntityEndTurnEvent>,  
    //mut ev_try_move: EventWriter<EntityTryMoveEvent>,
    //player_query: Query<(Entity, Has<Player>)>,   // no entity at the end? // TO DELETE?
    player_query: Query<Entity, With<Player>>,
    buttons: Res<ButtonInput<MouseButton>>,
    res_cursor: Res<Cursor>,    //TODO : On click event?
    mut ev_on_click: EventWriter<OnClickEvent>
){
    //println!("Checking if combat input...!");
    if keys.just_pressed(KeyCode::KeyT) {
        let Ok(result) = player_query.get_single() else { return };
        let entity = result;    //result.0 autrefois
        ev_endturn.send(EntityEndTurnEvent {entity});
        println!("Player asked for End of round for {:?}.", entity);
    }
    if buttons.just_released(MouseButton::Left) {
        let Ok(result) = player_query.get_single() else { return };
        let entity = result;    //result.0 autrefois
        let destination = res_cursor.grid_position;

        println!("Click !");
        ev_on_click.send(OnClickEvent { entity: entity, tile: destination });

        /* 
        println!("Clic to move!");
        ev_try_move.send(EntityTryMoveEvent {entity: entity, destination: destination});
        */

    }
}


/// Player clicked on a tile.
pub fn on_click_action(
    mut ev_onclick: EventReader<OnClickEvent>,
    mut ev_try_move: EventWriter<EntityTryMoveEvent>,
    action_infos: Res<ActionInfos>,
){
    for _event in ev_onclick.read() {
        let path = action_infos.path.clone();
        let Some(entity) = action_infos.entity else { continue };
        let Some(path) = path else { continue };

        println!("On clic action: OK. Send event.");
        ev_try_move.send(EntityTryMoveEvent {entity: entity, path: path, target: action_infos.target });

    }
}