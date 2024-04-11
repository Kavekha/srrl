use bevy::{input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel}, prelude::*};

use crate::{game::{combat::{action_infos::ActionInfos, components::{AttackType, WantToForfeit}, events::{RefreshActionCostEvent, WantToHitEvent}}, gamelog::LogEvent, 
    manager::{change_state_messages::{ChangeGameStateRunningMessage, ChangeGameStateUnavailableMessage}, 
    menu_messages::{CloseMenuMessage, OpenInGameMenuOpenMessage}, MessageEvent}}, menu_builders::ScrollingList};

use super::{components::WantToMoveEvent, Cursor, Player};


// 0.19d : Removal Abilities for now.
pub fn player_choose_action_input(
    mut action_infos: ResMut<ActionInfos>,
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_log: EventWriter<LogEvent>,
) {
    if keys.just_pressed(KeyCode::Digit1) {
        action_infos.attack = Some(AttackType::MELEE);
        ev_log.send(LogEvent {entry: format!("Now in Melee mode.")});  
    }
    if keys.just_pressed(KeyCode::Digit2) {
        action_infos.attack = Some(AttackType::RANGED);
        ev_log.send(LogEvent {entry: format!("Now in Targeting mode.")});       
    }
}


// Recalcule tout ActionInfos 
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
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    player_query: Query<Entity, With<Player>>,
    buttons: Res<ButtonInput<MouseButton>>,
    action_infos: ResMut<ActionInfos>,  // Contient le type d'attaque utilisé..
    res_cursor: Res<Cursor>,    //TODO : On click event?
    mut ev_want_to_hit: EventWriter<WantToHitEvent>,
    mut ev_want_to_move: EventWriter<WantToMoveEvent>
){
    //println!("Checking if combat input...!");
    if keys.just_pressed(KeyCode::KeyT) {
        let Ok(result) = player_query.get_single() else { return };     // TODO si on conserve action_infos, utiliser l'entité de ActionInfos?
        let entity = result;    //result.0 autrefois
        commands.entity(entity).insert(WantToForfeit { entity});
        //ev_endturn.send(EntityEndTurnEvent {entity});
        println!("Player asked for End of round for {:?}.", entity);
    }
    if buttons.just_released(MouseButton::Left) {
        let Ok(result) = player_query.get_single() else { return };
        let entity = result;    //result.0 autrefois
        let destination = res_cursor.grid_position;

        println!("Click !");
        match &action_infos.attack {
            None => {
                ev_want_to_move.send(WantToMoveEvent { entity: entity, tile: destination});
            },
            Some(attack_type) => {
                match attack_type {
                    AttackType::MELEE => {
                        ev_want_to_move.send(WantToMoveEvent { entity: entity, tile: destination});
                    },
                    AttackType::RANGED => {
                        ev_want_to_hit.send(WantToHitEvent { source: entity, target: destination});
                    }
                };
            },
            //_ => println!("Not combat_input.")
        };
    }
}


// 0.16.1
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