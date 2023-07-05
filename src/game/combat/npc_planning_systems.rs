use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{game::{pieces::components::{Npc, Occupier}, tileboard::components::BoardPosition, player::Player}, vectors::{Vector2Int, find_path}, map_builders::map::Map};

use super::{components::{CombatInfos, ActionPoints}, events::{EntityEndTurnEvent, Turn, EntityTryMoveEvent}};


/// NPC : Generate / Choice to forfeit their turn.
pub fn plan_action_forfeit(
    combat_info: Res<CombatInfos>,
    query_npc: Query<(Entity, &ActionPoints, &Turn), With<Npc>>,
    mut ev_endturn: EventWriter<EntityEndTurnEvent>,
){
    //println!("Planning forfeit...");
    let Some(_entity) = combat_info.current_entity else { return };  //TODO : Toujours necessaire avec le Component Turn?
    for (entity, _action_points, _turn) in query_npc.iter() {
        //TODO : Dans quelles circonstances un NPC decide de Forfeit.
        //println!("planning: Entity is a NPC.");
        ev_endturn.send(EntityEndTurnEvent {entity})     
    }  
}


/// NPC preconditions to resolve their goal.
pub struct NpcGoal {
    pub can_see_target: bool,
    pub have_path_to_target: bool,
    pub path: Option<VecDeque<Vector2Int>>    
}

pub fn npc_planning(
    combat_info: Res<CombatInfos>,
    query_npc: Query<(&ActionPoints, &BoardPosition), (With<Npc>, With<Turn>)>,
    query_player: Query<&BoardPosition, With<Player>>,
    mut ev_endturn: EventWriter<EntityEndTurnEvent>,
    query_occupied: Query<&BoardPosition, With<Occupier>>,
    board: Res<Map>,
    mut ev_try_move: EventWriter<EntityTryMoveEvent>
) {
    let Some(entity) = combat_info.current_entity else { return };
    let Ok(entity_infos) = query_npc.get(entity) else { return };
    let (npc_action_points, npc_position) = entity_infos;
    let Ok(player_position) = query_player.get_single() else { return };
   
    // Can see player?
    // Path to player?
    let path_to_player = find_path(
        npc_position.v,
        player_position.v, 
        &board.entity_tiles.keys().cloned().collect(), 
        &query_occupied.iter().map(|p| p.v).collect(),
        true,
    );

    let mut forfeit = false;

    // Impossible d'atteindre le joueur.
    if path_to_player.is_none() { 
        forfeit = true;
    }
     // Pas assez de PA pour atteindre le joueur.
     else if let Some(path) = path_to_player {
        if path.len() as u32  > npc_action_points.current {
            forfeit = true;
        } else {
            ev_try_move.send(EntityTryMoveEvent {entity: entity, path: path.clone(), target: Some(player_position.v) });
        }
     };

     if forfeit {
        ev_endturn.send(EntityEndTurnEvent {entity})
     }

}