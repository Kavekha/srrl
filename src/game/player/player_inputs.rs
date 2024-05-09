use bevy::{input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel}, prelude::*};

use crate::{
    game::{combat::{action_infos::{ActionInfos, CharacterAction}, 
    combat_system::components::{AttackType, WantToForfeit}, 
    events::{RefreshActionCostEvent, WantToHitEvent}}, gamelog::LogEvent, 
    manager::{change_state_messages::{ChangeGameStateRunningMessage, ChangeGameStateUnavailableMessage}, 
    menu_messages::{CloseMenuMessage, OpenInGameMenuOpenMessage}, MessageEvent},
    tileboard::components::BoardPosition}, globals::STANDARD_TILE_SIZE, 
    map_builders::map::Map, 
    menu_builders::components::ScrollingList, 
    vectors::Vector2Int};

use super::{components::WantToMoveEvent, Cursor, Player};


fn get_grid_position_debug(
    x: f32,
    y: f32
) -> Vector2Int {
    let mut grid_x = x;
    let mut grid_y= y - (y * 2.0);
    grid_x +=  (STANDARD_TILE_SIZE / 2) as f32;
    grid_y += (STANDARD_TILE_SIZE / 2) as f32;

    grid_x = grid_x / STANDARD_TILE_SIZE as f32;
    grid_y = grid_y / STANDARD_TILE_SIZE as f32;

    Vector2Int{x:grid_x as i32, y:grid_y as i32}

}

pub fn debug_info_on_click (
    mut res_cursor: ResMut<Cursor>,
    window_query: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    query_player_pos: Query<&BoardPosition, With<Player>>,
    buttons: Res<ButtonInput<MouseButton>>,
    //player_view_q: Query<&View>,
    player_q: Query<Entity, With<Player>>,
    board: Res<Map>,
) {
    if buttons.just_released(MouseButton::Right) {
        let Ok((camera, camera_transform)) = camera_q.get_single() else { return };
        if let Some(world_position) = window_query.single().cursor_position() 
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())      
        {
                res_cursor.world_position = Vec3 {x:world_position.x, y:world_position.y, z:0.0};
                res_cursor.grid_position = get_grid_position_debug(world_position.x, world_position.y);
        }
        res_cursor.screen_position = camera.world_to_viewport(camera_transform, res_cursor.world_position);
        let Ok(player_entity) = player_q.get_single() else { return };
        let Ok(_player_position) = query_player_pos.get(player_entity) else { return };
        //let Ok(player_view) = player_view_q.get(player_entity) else { return };

        // 0.20n : Si on connait, on peut cliquer.
        // !! TOCHECK : a voir ce qui se passe si on clique à un endroit où se trouve un npc mais que l'endroit n'est pas visible mais seulement connu.
        if board.is_revealed(res_cursor.grid_position.x, res_cursor.grid_position.y) {
        //if player_view.visible_tiles.contains(&res_cursor.grid_position) {
            info!("Clic at {:?} : this position is known and maybe visible.",res_cursor.grid_position);
        } else {
            info!("Clic at {:?} : this position is NOT known, so NOT visible.", res_cursor.grid_position);
        }
    }
}

// 0.19d : Removal Abilities for now.
pub fn player_choose_action_input(
    mut action_infos: ResMut<ActionInfos>,
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_log: EventWriter<LogEvent>,
    mut ev_refresh_action: EventWriter<RefreshActionCostEvent>,
) {
    if keys.just_pressed(KeyCode::Digit1) {
        action_infos.attack = Some(AttackType::MELEE);
        ev_log.send(LogEvent {entry: format!("Now in Melee mode.")});  
        ev_refresh_action.send(RefreshActionCostEvent);
        
    }
    if keys.just_pressed(KeyCode::Digit2) {
        action_infos.attack = Some(AttackType::RANGED);
        ev_log.send(LogEvent {entry: format!("Now in Targeting mode.")});    
        ev_refresh_action.send(RefreshActionCostEvent);   
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
/// 0.20j On s'assure que le clic soit dans la view.
/// 0.20n On retire l'intelligence qui se trouve desormais dans ActionInfos.
pub fn combat_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    player_query: Query<Entity, With<Player>>,
    buttons: Res<ButtonInput<MouseButton>>,
    action_infos: ResMut<ActionInfos>,  // Contient le type d'attaque utilisé..
    res_cursor: Res<Cursor>,    //TODO : On click event?
    mut ev_want_to_hit: EventWriter<WantToHitEvent>,
    mut ev_want_to_move: EventWriter<WantToMoveEvent>,
){
    //println!("Checking if combat input...!");
    if keys.just_pressed(KeyCode::KeyT) {
        let Ok(result) = player_query.get_single() else { return };     // TODO si on conserve action_infos, utiliser l'entité de ActionInfos?
        let entity = result;    //result.0 autrefois
        commands.entity(entity).insert(WantToForfeit);
        //ev_endturn.send(EntityEndTurnEvent {entity});
        println!("Player asked for End of round for {:?}.", entity);
    }
    if buttons.just_pressed(MouseButton::Left) {           //just_released before 0.20o
        let Ok(result) = player_query.get_single() else { return };
        let entity = result;    //result.0 autrefois
        let destination = res_cursor.grid_position;

        info!("Click ! {:?}", action_infos.available_action); 
        match &action_infos.available_action {
            CharacterAction::NONE => {},
            CharacterAction::WAITING => {},
            CharacterAction::CANTSEE => {},
            CharacterAction::MOVING => { 
                ev_want_to_move.send(WantToMoveEvent { entity: entity, tile: destination}); 
            },
            CharacterAction::PUNCHING => { ev_want_to_move.send(WantToMoveEvent { entity: entity, tile: destination}); },
            CharacterAction::TARGETING => { ev_want_to_hit.send(WantToHitEvent { source: entity, target: destination}); },
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