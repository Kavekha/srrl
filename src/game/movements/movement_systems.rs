use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{game::{combat::{components::{ActionPoints, AttackType, WantToHit}, action_infos::ActionInfos, events::{RefreshActionCostEvent, Turn}, rules::{consume_actionpoints, AP_COST_MELEE, AP_COST_MOVE}}, movements::components::MoveTo, player::{components::WantToMoveEvent, Player}, tileboard::components::BoardPosition, ui::ReloadUiEvent}, vectors::Vector2Int};
use crate::engine::animations::events::AnimateEvent;

use super::components::WantToMove;


/// 0.19b refacto
pub fn on_want_to_move_event(
    mut commands: Commands,
    mut ev_want_to_move: EventReader<WantToMoveEvent>,
    action_infos: Res<ActionInfos>,
){
    for event in ev_want_to_move.read() {
        println!("Move Event recu");
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
    //mut ev_try_attack: EventWriter<EntityHitTryEvent>,
    mut ev_refresh_action: EventWriter<RefreshActionCostEvent>,
    //mut ev_move: EventWriter<EntityMoveEvent>,
){
    for (entity, want_move) in want_move_q.iter() {
        println!("Want To Move component.");
        commands.entity(entity).remove::<WantToMove>();

        let Ok(action_points) = actions_q.get(entity) else { continue };
        if action_points.current < AP_COST_MOVE { continue };

        // Target check
        let Some(destination) = want_move.path.get(0) else { continue };
        if let Some(current_target) = want_move.target {
            if current_target == * destination {
                if action_points.current < AP_COST_MELEE { continue };
                println!("J'attaque ma cible!!!");
                //ev_try_attack.send( EntityHitTryEvent {entity: want_move.entity, target: current_target});
                commands.entity(want_move.entity).insert(WantToHit { mode: AttackType::MELEE, target: destination.clone() });
                continue
            }
        }  
        ev_refresh_action.send(RefreshActionCostEvent);

        let path = want_move.path.clone();
        //ev_move.send(EntityMoveEvent {entity: want_move.entity, path: path, target: want_move.target});
        commands.entity(want_move.entity).insert(MoveTo { path: path, target: want_move.target});   //TODO: Normalement on a plus de "Some()" à ce moment là, hors on est en Option.
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
){
    for (entity, movement) in move_q.iter() {
        commands.entity(entity).remove::<MoveTo>();

        println!("{:?} : Je bouge!", entity);
        let Ok(entity_infos) = query_character_turn.get_mut(entity) else { 
            //println!("ActionMove: Je n'ai pas les infos Entité");   // TODO : Quand Action_entity_try_move pose le component MovePath, le Query action_entity_move ne le recupere pas pour le moment (asynchrone?)
            continue };
        let (entity, mut action_points,mut board_position, is_player) = entity_infos;  

        let mut path = movement.path.clone();
        let destination = path.pop_front();
        let Some(new_position) = destination.clone() else { break };
        
        board_position.v = new_position;
        //ev_try_move.send(EntityTryMoveEvent {entity: entity, path: path, target: movement.target});
        commands.entity(entity).insert(WantToMove { entity: entity, path: path, target: movement.target});

        consume_actionpoints(&mut action_points, AP_COST_MOVE);
        //action_points.current = action_points.current.saturating_sub(AP_COST_MOVE);
        if is_player.is_some() {
            ev_interface.send(ReloadUiEvent);
        }

        ev_refresh_action.send(RefreshActionCostEvent);

        let mut path_animation: VecDeque<Vector2Int> = VecDeque::new();
        path_animation.push_back(new_position);
        ev_animate.send(AnimateEvent { entity: entity, path: path_animation });

    }
}
