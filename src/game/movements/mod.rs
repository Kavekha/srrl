use std::collections::VecDeque;

use bevy::prelude::*;

use crate::game::combat::{AP_COST_MELEE, AP_COST_MOVE};
use crate::engine::animations::events::AnimateEvent;
use crate::vectors::Vector2Int;

use super::combat::rules::consume_actionpoints;
use super::{
    combat::{components::ActionPoints, events::{EntityHitTryEvent, EntityMoveEvent, EntityTryMoveEvent, RefreshActionCostEvent, Turn}}, 
    player::Player, tileboard::components::BoardPosition, ui::ReloadUiEvent};




pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EntityTryMoveEvent>()         // Tente deplacement: check si target ou simple mouvement.
            .add_event::<EntityMoveEvent>()            // Se deplace.
            // Check des actions demandées.
            .add_systems(Update, action_entity_try_move)    //.run_if(in_state(GameState::Running)).in_set(CombatSet::Logic))
            .add_systems(Update, action_entity_move)    //.run_if(in_state(GameState::Running)).in_set(CombatSet::Tick).after(action_entity_try_move))
                
            ;
    }
}



/// Test de l'action Move.
pub fn action_entity_try_move(
    mut ev_try_move: EventReader<EntityTryMoveEvent>,
    mut ev_move: EventWriter<EntityMoveEvent>,
    mut ev_try_attack: EventWriter<EntityHitTryEvent>,
    query_actions: Query<&ActionPoints>,
    mut ev_refresh_action: EventWriter<RefreshActionCostEvent>,
){
    for event in ev_try_move.read() {
        println!("Action entity try move event received.");

        let Ok(action_points) = query_actions.get(event.entity) else { continue };
        if action_points.current < AP_COST_MOVE { continue };

        // Target check
        let Some(destination) = event.path.get(0) else { continue };
        if let Some(current_target) = event.target {
            if current_target == * destination {
                if action_points.current < AP_COST_MELEE { continue };
                println!("J'attaque ma cible!!!");
                ev_try_attack.send( EntityHitTryEvent {entity: event.entity, target: current_target});
                continue
            }
        }  
        ev_refresh_action.send(RefreshActionCostEvent);

        let path = event.path.clone();
        ev_move.send(EntityMoveEvent {entity: event.entity, path: path, target: event.target});
    }
}


/// Gestion de l'action Move.
pub fn action_entity_move(
    mut query_character_turn: Query<(Entity, &mut ActionPoints, &mut BoardPosition, Option<&Player>), With<Turn>>,
    mut ev_move: EventReader<EntityMoveEvent>,
    mut ev_interface: EventWriter<ReloadUiEvent>,
    mut ev_animate: EventWriter<AnimateEvent>,
    mut ev_try_move: EventWriter<EntityTryMoveEvent>,    
    mut ev_refresh_action: EventWriter<RefreshActionCostEvent>,
){    
    for event in ev_move.read() {
        let Ok(entity_infos) = query_character_turn.get_mut(event.entity) else { 
            //println!("ActionMove: Je n'ai pas les infos Entité");   // TODO : Quand Action_entity_try_move pose le component MovePath, le Query action_entity_move ne le recupere pas pour le moment (asynchrone?)
            continue };
        let (entity, mut action_points,mut board_position, is_player) = entity_infos;  

        let mut path = event.path.clone();
        let destination = path.pop_front();
        let Some(new_position) = destination.clone() else { break };
        
        board_position.v = new_position;
        ev_try_move.send(EntityTryMoveEvent {entity: event.entity, path: path, target: event.target});

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
