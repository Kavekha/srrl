use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{game::{
    combat::{
        action_infos::ActionInfos, 
        combat_system::components::{ActionPoints, AttackType, WantToHit}, 
        events::{RefreshActionCostEvent, Turn},
         rules::{consume_actionpoints, AP_COST_MELEE, AP_COST_MOVE}
    }, 
    movements::components::MoveTo, 
    player::{components::WantToMoveEvent, Player}, 
    tileboard::components::BoardPosition, ui::events::ReloadUiEvent, visibility::components::{ComputeFovEvent, View}}, vectors::Vector2Int
};
use crate::engine::animations::events::AnimateEvent;

use super::components::{CancelMoveEvent, MoveEvent, WantToMove};


// 0.20l Pouvoir annuler un deplacement en cours de PJ.
pub fn interrupting_movement(
    mut commands:Commands,
    mut ev_cancel_move: EventReader<CancelMoveEvent>,
 ) {
    for event in ev_cancel_move.read() {
        commands.entity(event.entity).remove::<WantToMove>();
        commands.entity(event.entity).remove::<MoveTo>();
        println!("Event Cancel Move!");
    }    
 }
 

/// 0.19b refacto
pub fn on_want_to_move_event(
    mut commands: Commands,
    mut ev_want_to_move: EventReader<WantToMoveEvent>,
    action_infos: Res<ActionInfos>,
){
    for event in ev_want_to_move.read() {
        //info!("Move Event recu");
        let path = action_infos.path.clone();
        let Some(entity) = action_infos.entity else { continue };
        let Some(path) = path else { continue };

        commands.entity(event.entity).insert(WantToMove { entity: entity, path: path, target: action_infos.target});
    }
}

// 0.19b
pub fn entity_want_to_move(
    mut commands: Commands,
    want_move_q: Query<(Entity, &WantToMove)>,
    actions_q: Query<&ActionPoints>,    
    mut ev_refresh_action: EventWriter<RefreshActionCostEvent>,
){
    let mut to_remove = Vec::new();
    for (entity, want_move) in want_move_q.iter() {
        //info!("{:?} Want To Move.", entity);
        to_remove.push(entity);

        let Ok(action_points) = actions_q.get(entity) else { continue };
        if action_points.current < AP_COST_MOVE {
            //info!("NPC {:?} wanted to move but doesnt have the AP.", entity);
             continue };

        // Target check
        let Some(destination) = want_move.path.get(0) else { 
            //info!("NPC {:?} doesnt have destination / want move path get 0.", entity);
            continue };
        if let Some(current_target) = want_move.target {
            //info!("NPC {:?} want to hit target.", entity);
            if current_target == * destination {
                if action_points.current < AP_COST_MELEE { continue };
                //info!("J'attaque ma cible!!!");
                //ev_try_attack.send( EntityHitTryEvent {entity: want_move.entity, target: current_target});
                commands.entity(want_move.entity).insert(WantToHit { mode: AttackType::MELEE, target: destination.clone() });
                continue
            }
        }  
        ev_refresh_action.send(RefreshActionCostEvent);

        let path = want_move.path.clone();
        //ev_move.send(EntityMoveEvent {entity: want_move.entity, path: path, target: want_move.target});
        commands.entity(want_move.entity).insert(MoveTo { path: path, target: want_move.target});   //TODO: Normalement on a plus de "Some()" à ce moment là, hors on est en Option.
        //info!("{:?} now really move.", entity);
    }
    for entity in to_remove {        
        commands.entity(entity).remove::<WantToMove>();
    }
}



// 0.19b
pub fn entity_move_to(
    mut commands: Commands,
    move_q: Query<(Entity, &MoveTo)>,
    mut query_character_turn: Query<(Entity, &mut ActionPoints, &mut BoardPosition, Option<&Player>), With<Turn>>,    
    mut ev_interface: EventWriter<ReloadUiEvent>,
    mut ev_refresh_action: EventWriter<RefreshActionCostEvent>,
    mut ev_animate: EventWriter<AnimateEvent>,
    mut ev_compute_fov: EventWriter<ComputeFovEvent>,
    mut ev_move_event: EventWriter<MoveEvent>,
    view_q:Query<&View>,
    entity_player_q:Query<Entity, With<Player>>
){
    let mut to_remove = Vec::new();
    let mut player_view = None;
    if let Ok(entity_player) = entity_player_q.get_single() {
        if let Ok(view) = view_q.get(entity_player) {
            player_view = Some(view);   
        }        
    }
    for (entity, movement) in move_q.iter() {
        to_remove.push(entity);

        //info!("{:?} : Je bouge!", entity);
        let Ok(entity_infos) = query_character_turn.get_mut(entity) else { 
            //println!("ActionMove: Je n'ai pas les infos Entité");   // TODO : Quand Action_entity_try_move pose le component MovePath, le Query action_entity_move ne le recupere pas pour le moment (asynchrone?)
            continue };
        let (entity, mut action_points,mut board_position, is_player) = entity_infos;  

        let mut path = movement.path.clone();
        let destination = path.pop_front();
        let Some(new_position) = destination.clone() else { break };
        
        ev_move_event.send(MoveEvent { entity: entity, previous: board_position.v, next: new_position});    // 0.20k

        board_position.v = new_position;
        ev_refresh_action.send(RefreshActionCostEvent);
        if is_player.is_some() {
            ev_compute_fov.send(ComputeFovEvent);   // 0.20k => Uniquement pour le PJ, les NPC sont couverts par le MoveEvent.
        }      
        
        commands.entity(entity).insert(WantToMove { entity: entity, path: path, target: movement.target});

        consume_actionpoints(&mut action_points, AP_COST_MOVE);
        //action_points.current = action_points.current.saturating_sub(AP_COST_MOVE);

        let mut wait_anim = false;
        match player_view {
            Some(p_view) => {
                if p_view.visible_tiles.contains(&new_position) {
                    wait_anim = true;
                }
            },
            None => {},
        } 
        
        if is_player.is_some() {
            ev_interface.send(ReloadUiEvent);                
        }            
        
        let mut path_animation: VecDeque<Vector2Int> = VecDeque::new();
        path_animation.push_back(new_position);
        ev_animate.send(AnimateEvent { entity: entity, path: path_animation, wait_anim: wait_anim });
    }
    for entity in to_remove {        
        commands.entity(entity).remove::<MoveTo>();
    }
}
