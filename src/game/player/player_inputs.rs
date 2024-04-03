use bevy::{prelude::*, input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel}};

use crate::{game::{combat::{event_systems::ActionInfos, events::{EntityEndTurnEvent, EntityTryMoveEvent, RefreshActionCostEvent}}, gamelog::LogEvent, manager::{change_state_messages::{ChangeGameStateRunningMessage, ChangeGameStateUnavailableMessage}, menu_messages::{CloseMenuMessage, OpenInGameMenuOpenMessage}, MessageEvent}, player::cursor::CursorMode}, menu_builders::ScrollingList};

use super::{components::OnClickEvent, Cursor, Player};



// 0.18 : ranged attack at last.
pub fn player_choose_action_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut res_cursor: ResMut<Cursor>,
    mut ev_log: EventWriter<LogEvent>,
) {
    if keys.just_pressed(KeyCode::Digit1) {
        match res_cursor.mode {
            CursorMode::MELEE => {
                ev_log.send(LogEvent {entry: format!("Already in Melee mode.")});    //TO CHANGE
            },
            _ => {
                res_cursor.mode = CursorMode::MELEE;
                ev_log.send(LogEvent {entry: format!("Now in Melee mode.")});                
            },
        };        
        println!("Choosing Melee combat.");
    }
    if keys.just_pressed(KeyCode::Digit2) {
        match res_cursor.mode {
            CursorMode::TARGET => {
                ev_log.send(LogEvent {entry: format!("Already in Targeting mode.")});    //TO CHANGE
            },
            _ => {
                res_cursor.mode = CursorMode::TARGET;
                ev_log.send(LogEvent {entry: format!("Now in Targeting mode.")});                
            },
        };   
        println!("Choosing Ranged combat.");
    }
}


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
        ev_on_click.send(OnClickEvent { entity: entity, tile: destination, mode: res_cursor.mode.clone() });           

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
    for event in ev_onclick.read() {
        match event.mode {
            CursorMode::MELEE => {
                let path = action_infos.path.clone();
                let Some(entity) = action_infos.entity else { continue };
                let Some(path) = path else { continue };
        
                println!("On clic action: OK. Send event.");
                ev_try_move.send(EntityTryMoveEvent {entity: entity, path: path, target: action_infos.target });
            },
            CursorMode::TARGET => {
                println!("Targeting here");
            }
        };
    }
}


//DEBUG / TEST 0.16.1
pub fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}