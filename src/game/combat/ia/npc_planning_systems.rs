use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{game::{combat::{components::{ActionPoints, CombatInfos, IsDead}, events::{EntityEndTurnEvent, Turn}, rules::{AP_COST_MELEE, AP_COST_MOVE}}, movements::components::WantToMove, pieces::components::{Npc, Occupier}, player::Player, tileboard::components::BoardPosition}, map_builders::map::Map, vectors::{find_path, Vector2Int}};


// ===> DOCUMENTATION 0.19h
/*
Conception:
* Chaque NPC regarde s'ils voient le PJ.
    * Chaque NPC qui voient le PJ communiquent avec les autres NPC pour les en informer.
* Si NPC est mélée only, regarde s'il peut porter au moins un coup.
    * S'il ne peut pas porter au moins un coup, regarde si d'autres autour de lui peuvent frapper: si c'est le cas, il approche quand même.
    * Sinon, il essait de rejoindre d'autres NPC
    * Sinon il s'approche mais petit à petit.
    * Sinon il se rends vers la sortie.

----------------------------------------------------------------------------------------------------
| 0.19h    | 0.3 |
| 0.13     | 0.2 | IA  planifie, attaque si AP, bouge si AP, abandonne si rien à faire.
| 0.6      | 0.1 | NPC poursuivent le joueur.
------------------------------------------------------------------------------------------------
 */



/// NPC : Generate / Choice to forfeit their turn.
#[allow(dead_code)]
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
        ev_endturn.send(EntityEndTurnEvent {entity});
    }  
}


/// NPC preconditions to resolve their goal.
#[allow(dead_code)]
pub struct NpcGoal {
    pub can_see_target: bool,
    pub have_path_to_target: bool,
    pub path: Option<VecDeque<Vector2Int>>    
}

pub fn npc_planning(
    mut commands: Commands,
    combat_info: Res<CombatInfos>,
    query_npc: Query<(&ActionPoints, &BoardPosition), (With<Npc>, With<Turn>)>,
    query_player: Query<&BoardPosition, (With<Player>, Without<IsDead>)>,       // TODO : si pas Without<IsDead>, crash car jeu quitté suite au Game Over sans que les NPC ne finissent leur tour.
    mut ev_endturn: EventWriter<EntityEndTurnEvent>,
    query_occupied: Query<&BoardPosition, With<Occupier>>,
    board: Res<Map>,
) {
    //println!("Planing: First step:");
    
    let Some(entity) = combat_info.current_entity else { return };
    let Ok(entity_infos) = query_npc.get(entity) else { return };
    let (npc_action_points, npc_position) = entity_infos;
    let Ok(player_position) = query_player.get_single() else { return };
   
    println!("Planing: Check: OK for {:?}", entity);

    // Can do something?
    if npc_action_points.current < AP_COST_MELEE { 
        ev_endturn.send(EntityEndTurnEvent {entity}); 
        return
    }
    println!("Planing: {:?} Has enough AP : {:?}", entity, npc_action_points.current);

    let path_to_player = find_path(
        npc_position.v,
        player_position.v, 
        &board.entity_tiles.keys().cloned().collect(), 
        &query_occupied.iter().map(|p| p.v).collect(),
        true,
    );
    if path_to_player.is_none() { 
        ev_endturn.send(EntityEndTurnEvent {entity}); 
        return
    }
    println!("Planning: {:?} has a path to the target.", entity);

    if let Some(path) = path_to_player {
        // Est ce que je peux toucher le PJ?
        if (path.len() as u32 * AP_COST_MOVE) + AP_COST_MELEE < npc_action_points.current {
            println!("Planning: My target is close enough to hit: {:?}", entity);
            //ev_try_move.send(EntityTryMoveEvent { entity: entity, path: path.clone(), target: Some(player_position.v) });     // Avant 0.19b
            commands.entity(entity).insert(WantToMove { entity: entity, path: path, target: Some(player_position.v)});
            return
        }
        // Si je ne peux pas le toucher, je m'approche quand même.
        let mut new_path = VecDeque::new();
        let mut ap = npc_action_points.current.clone();
        let mut count = 0;
        while ap > AP_COST_MOVE * 5 {
            if let Some(new_position) = path.get(count) {
                let next_position = new_position.clone();
                count += 1;
                ap -= 1;
                new_path.push_back(next_position);
            } else {
                println!("Planning: Limite du path atteinte.");
                break
            }
        }
        if !new_path.is_empty() {
            println!("Planning: Moving closer: {:?}", entity);
            //ev_try_move.send(EntityTryMoveEvent { entity: entity, path: new_path, target: Some(player_position.v) });
            commands.entity(entity).insert(WantToMove { entity: entity, path: new_path, target: Some(player_position.v)});
            return
        }
    }
    println!("Planning: {:?} has nothing better than forfeit.", {entity});
    ev_endturn.send(EntityEndTurnEvent {entity});

}