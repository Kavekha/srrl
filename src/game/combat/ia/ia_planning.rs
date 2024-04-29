use bevy::prelude::*;

use crate::{
    game::{combat::{combat_system::components::{IsDead, WantToForfeit},
    events::Turn, rules::VISIBILITY_RANGE_NPC}, commons::is_in_sight, game_generation::character_creation::components::{NavigationNode, Npc, Occupier, Walk}, movements::components::WantToMove, tileboard::components::BoardPosition}, map_builders::map::Map, vectors::find_path};

use super::components::{HasShareInfos, Knowledge, PlanFlee, PlanInformAllies, PlanMove, PlanSearch};


pub fn planning_inform_allies(
    mut commands: Commands,
    npc_entity_fighter_q: Query<(Entity, &BoardPosition, &PlanInformAllies), (With<Npc>, With<Turn>, Without<IsDead>)>,
    mut npc_position_q: Query<(Entity, &BoardPosition, &mut Knowledge), With<Npc>>,
    board: Res<Map>,
){
    for (entity, position, inform) in npc_entity_fighter_q.iter() {
        for (npc_entity, npc_position, mut npc_knowledge) in npc_position_q.iter_mut() {
            // TODO : Enregistrer les alliés proches?
            if let Ok(_) = is_in_sight(&board, &position.v, &npc_position.v, VISIBILITY_RANGE_NPC) {
                match npc_knowledge.player_last_seen {
                    Some(_) => {
                        info!("Npc already has this information.");
                    },
                    None => { 
                        info!("Npc share info with nearest allies");
                        npc_knowledge.player_last_seen = Some(inform.target_position.clone());
                        commands.entity(npc_entity).insert(HasShareInfos);
                    }                       
                }                
            }
        }
        commands.entity(entity).insert(HasShareInfos);
    }
}

// Old version, a adapter => si echec on ne sort pas de la boucle.
pub fn planning_approaching( 
    mut commands: Commands,
    npc_entity_fighter_q: Query<(Entity, &BoardPosition, &PlanMove), (With<Npc>, With<Turn>, Without<IsDead>, With<Walk>)>,   
    board: Res<Map>,
    query_occupied: Query<&BoardPosition, With<Occupier>>,
) {
    let mut to_remove_plan_move = Vec::new();
    for (npc_entity, npc_position, npc_plan) in npc_entity_fighter_q.iter() {
        // Pas de Goal, on a déjà determiné cela avant.

        let path_to_destination = find_path(
            npc_position.v,
            npc_plan.destination, 
            &board.entity_tiles.keys().cloned().collect(), 
            &query_occupied.iter().map(|p| p.v).collect(),
            true,  // Obligé de l'avoir en true, sinon on considère que pas de route pour s'y rendre.
        );
        
        if let Some(mut path) = path_to_destination {
            //println!("NPC {:?} J'ai planifié un chemin pour moi.", npc_entity);
            let _remove_last_destination = path.pop_back(); // On fait ça sinon le perso se deplacera sur sa cible sans l'attaquer. 
            commands.entity(npc_entity).insert(WantToMove { entity: npc_entity, path: path, target: None}); //On ne veut pas attaquer.  Some(npc_plan.destination)});    
        } else {
            //println!("Pas de chemin pour moi.");
            commands.entity(npc_entity).insert(WantToForfeit);  // Securité pour ne pas rester bloqué.
        }
        // Retrait du PlanMove sinon on ne refait plus le check View.   // REMINDER: C'etait très cool !
        to_remove_plan_move.push(npc_entity);
    }
    for entity in to_remove_plan_move {
        commands.entity(entity).remove::<PlanMove>();   
    }
}


pub fn planning_fleeing( 
    mut commands: Commands,
    npc_entity_fighter_q: Query<(Entity, &BoardPosition, &PlanFlee), (With<Npc>, With<Turn>, Without<IsDead>, With<Walk>)>,   
    board: Res<Map>,
    query_occupied: Query<&BoardPosition, With<Occupier>>,
    exit_position_q: Query<&BoardPosition, With<NavigationNode>>,
) {
    let mut to_remove_plan_move = Vec::new();
    for (npc_entity, npc_position, _) in npc_entity_fighter_q.iter() {
        //info!("Plan flee: exit found. have I a path to it?");
        to_remove_plan_move.push(npc_entity);        

        // Utilisé dans room_based_exits. 
        let mut farest_distance = 0;
        let mut farest_destination= npc_position.v;
        for room in exit_position_q.iter() {
            let distance = npc_position.v.clone().manhattan(room.v);
            if distance > farest_distance {
                farest_distance = distance;
                farest_destination = room.v; 
            }
        }

        if farest_destination != npc_position.v {
            // Je m'eloigne de ma destination vers la sortie.
            let path_to_destination = find_path(
                npc_position.v,
                farest_destination, 
                &board.entity_tiles.keys().cloned().collect(), 
                &query_occupied.iter().map(|p| p.v).collect(),
                true,  // Obligé de l'avoir en true, sinon on considère que pas de route pour s'y rendre.
            );
            
            if let Some(path) = path_to_destination {
                //let next_position = path.get(0).copied();
                //info!("I am {:?}, i'm at {:?} and my target is {:?}", npc_entity, npc_position.v, next_position);
                //println!("NPC {:?} J'ai planifié un chemin pour moi.", npc_entity);
                //info!("Plan flee: path to exit found.");
                commands.entity(npc_entity).insert(WantToMove { entity: npc_entity, path: path, target: None});    
            } else {
                //println!("Pas de chemin pour moi.");
                info!("Plan flee: No path to exit found, forfeit.");
                commands.entity(npc_entity).insert(WantToForfeit);  // Securité pour ne pas rester bloqué.
            }
        } else {
            info!("Plan flee: No exit found, forfeit.");
            for (npc_entity, _, _) in npc_entity_fighter_q.iter() {
                commands.entity(npc_entity).insert(WantToForfeit);  // Securité pour ne pas rester bloqué.
            }
        }
    }
    for entity in to_remove_plan_move {
        commands.entity(entity).remove::<PlanFlee>();   
    }
}


pub fn planning_searching( 
    mut commands: Commands,
    mut npc_entity_fighter_q: Query<(Entity, &BoardPosition, &PlanSearch, &mut Knowledge), (With<Npc>, With<Turn>, Without<IsDead>, With<Walk>)>,   
    board: Res<Map>,
    query_occupied: Query<&BoardPosition, With<Occupier>>,
    exit_position_q: Query<&BoardPosition, With<NavigationNode>>,
) {
    let mut to_remove_plan_move = Vec::new();

    for (npc_entity, npc_position, _, mut knowledge) in npc_entity_fighter_q.iter_mut() {
        info!("Plan flee: exit found. have I a path to it?");
        to_remove_plan_move.push(npc_entity);        

        if knowledge.last_visited_nodes.len() > 4 {
            knowledge.last_visited_nodes = Vec::new();
        }
        // Utilisé dans room_based_exits. 
        let mut nearest_distance = 0;
        let mut nearest_destination= npc_position.v;
        for room in exit_position_q.iter() {
            let distance = npc_position.v.clone().manhattan(room.v);
            if knowledge.last_visited_nodes.contains(&room.v) {
                info!("Npc {:?} a déjà visité le node {:?}", npc_entity, room);
                continue 
            } else {
                if distance > nearest_distance {
                    nearest_distance = distance;
                    nearest_destination = room.v; 
                }
            }
        }

        // Similaire à Flee et Exit de MapBuilder.
        if nearest_destination != npc_position.v {
            if nearest_distance < 5 {   // Min distance pour considerer qu'on ne reverifiera pas.
                knowledge.last_visited_nodes.push(nearest_destination); // Je me souviens de la destination.
            }            
            // Je m'eloigne de ma destination vers la sortie.
            let path_to_destination = find_path(
                npc_position.v,
                nearest_destination, 
                &board.entity_tiles.keys().cloned().collect(), 
                &query_occupied.iter().map(|p| p.v).collect(),
                true,  // Obligé de l'avoir en true, sinon on considère que pas de route pour s'y rendre.
            );
            
            if let Some(path) = path_to_destination {
                let next_position = path.get(0).copied();
                info!("I am {:?}, i'm at {:?} and I want to search to {:?}", npc_entity, npc_position.v, next_position);
                commands.entity(npc_entity).insert(WantToMove { entity: npc_entity, path: path, target: None});    
            } else {
                //println!("Pas de chemin pour moi.");
                info!("Plan Search: No Node found, forfeit.");
                commands.entity(npc_entity).insert(WantToForfeit);  // Securité pour ne pas rester bloqué.
            }
        } else {
            info!("Plan Search: No Node found, forfeit.");
            commands.entity(npc_entity).insert(WantToForfeit);  // Securité pour ne pas rester bloqué.
        }
    }
    for entity in to_remove_plan_move {
        commands.entity(entity).remove::<PlanSearch>();   
    }
}