use bevy::prelude::*;

use crate::{game::{
    combat::{combat_system::components::{ActionPoints, AttackType, IsDead, WantToForfeit, WantToHit}, events::Turn, ia::components::{PlanFlee, PlanMove, PlanSearch}, rules::{AP_COST_MELEE, AP_COST_MOVE, AP_COST_RANGED, LOW_HP_THRESHOLD, VISIBILITY_RANGE_NPC}}, commons::is_in_sight, movements::components::WantToMove, pieces::components::{Health, Melee, NavigationNode, Npc, Occupier, Ranged, Walk}, player::Player, tileboard::components::BoardPosition},
    map_builders::map::Map, vectors::find_path
};

use super::components::CheckGoal;


#[derive(Component, Debug)]
pub struct Planning {
    pub in_sight: bool,
    pub ap_for_range: bool,
    pub melee_range: bool,
    pub ap_for_melee: bool,
    pub low_health: bool,
    pub has_allies_nearby: bool,
    pub can_move: bool,
}
impl Planning {
    pub fn new() -> Planning {
        Planning {
            in_sight: false,
            ap_for_range: false,
            melee_range: false,
            ap_for_melee: false,
            low_health: false,
            has_allies_nearby: false,
            can_move: false,
        }
    }
    pub fn reset(&mut self) {
        self.in_sight= false;
        self.ap_for_range= false;
        self.melee_range= false;
        self.ap_for_melee= false;
        self.low_health= false;
        self.has_allies_nearby= false;
        self.can_move= false;
    }
}


// 0.20r : v1.1 : Les protections si on ne peut pas bouger sont dans les PlanMove pour le moment. C'est pas ouf.
// 0.20q : v1 : NOTE : On est très vulnerable à un problème entre l'estimation et la résolution, qui bloquera le jeu.
// TODO : Prevoir les actions, les jouer une à une jusqu'à ce qu'il n'y en ai plus.
pub fn planning_evaluate_actions(
    mut commands: Commands,
    npc_entity_fighter_q: Query<(Entity, &Planning), (With<Npc>, With<Turn>, Without<IsDead>)>,
    entity_player_q: Query<Entity, With<Player>>,
    position_q: Query<&BoardPosition>,

) {     
    let Ok(target) = entity_player_q.get_single() else { return };
    let Ok(target_position) = position_q.get(target) else { return };
    let mut to_remove = Vec::new();

    for (entity, planning) in npc_entity_fighter_q.iter() {
        info!("{:?} is planning -----------------", entity);
        info!("{:?}", planning);
        
                // Attaque à distance
        if planning.in_sight && planning.ap_for_range {
            info!("{:?} va attaquer sa cible à distance.", entity);
            commands.entity(entity).insert(WantToHit { mode: AttackType::RANGED, target: target_position.v });
        }
        // Melee 
        if planning.in_sight && planning.ap_for_melee && planning.melee_range {
            info!("{:?} va attaquer sa cible en melee.", entity);
            commands.entity(entity).insert(WantToHit { mode: AttackType::MELEE, target: target_position.v });
        }
        // Trop loin de la cible mais peut taper.
        if planning.in_sight && planning.ap_for_melee && planning.can_move {
            // TODO : Doit verifier s'il peut porter un coup ce tour.
            info!("{:?} va se rapprocher de sa cible pour l'attaquer en melee!", entity);
            commands.entity(entity).insert(PlanMove { destination: target_position.v}); 
        }
        // En vue, mais ne peut pas taper.
        if planning.in_sight && !planning.ap_for_melee && !planning.ap_for_range && planning.can_move {
            //s'eloigne
            info!("{:?} va s'éloigner", entity);
            commands.entity(entity).insert(PlanFlee { away_from: target_position.v}); 
        }
        // Ne voit pas la cible : la cherche.
        if !planning.in_sight && planning.can_move {
            info!("{:?} recherche sa cible.", entity);
            commands.entity(entity).insert(PlanSearch ); 
        } 
        to_remove.push(entity);  
    }
    for entity in to_remove {
        commands.entity(entity).remove::<Planning>(); 
    }
}


// 0.20q : v1 : NOTE : On est très vulnerable à un problème entre l'estimation et la résolution, qui bloquera le jeu.
// TODO : Prevoir les actions, les jouer une à une jusqu'à ce qu'il n'y en ai plus.
pub fn planning_evaluate_actions_v1(
    mut commands: Commands,
    npc_entity_fighter_q: Query<(Entity, &Planning), (With<Npc>, With<Turn>, Without<IsDead>)>,
    entity_player_q: Query<Entity, With<Player>>,
    position_q: Query<&BoardPosition>,

) {     
    let Ok(target) = entity_player_q.get_single() else { return };
    let Ok(target_position) = position_q.get(target) else { return };
    let mut to_remove = Vec::new();

    for (entity, planning) in npc_entity_fighter_q.iter() {
        info!("{:?} is planning -----------------", entity);
        info!("{:?}", planning);
        /*           
        in_sight: false,
            ap_for_range: false,
            melee_range: false,
            ap_for_melee: false,
            low_health: false,
            has_allies_nearby: false,
            can_move: false, */
        if planning.in_sight {
            // Est-ce que je peux attaquer?
            if planning.ap_for_range {
                info!("{:?} va attaquer sa cible à distance.", entity);
                commands.entity(entity).insert(WantToHit { mode: AttackType::RANGED, target: target_position.v });
                to_remove.push(entity);
                continue
            } else if planning.melee_range && planning.ap_for_melee {
                info!("{:?} va attaquer sa cible en melee.", entity);
                commands.entity(entity).insert(WantToHit { mode: AttackType::MELEE, target: target_position.v });
                to_remove.push(entity);                
                continue
            } else if planning.ap_for_melee && planning.can_move {
                info!("{:?} va se rapprocher de sa cible pour l'attaquer en melee!", entity);
                commands.entity(entity).insert(PlanMove { destination: target_position.v}); 
                to_remove.push(entity);  
                continue
            }
            // Si je ne peux pas taper: je m'eloigne
            if planning.can_move {
                info!("{:?} va s'éloigner", entity);
                commands.entity(entity).insert(PlanFlee { away_from: target_position.v}); 
                to_remove.push(entity);  
                continue
            }          
        } 
        if planning.can_move {
            info!("{:?} va se deplacer au hasard pour chercher sa cible.", entity);
            //TODO : Choisir une destination au hasard.
            commands.entity(entity).insert(PlanMove { destination: target_position.v}); 
                to_remove.push(entity);  
                continue
        }
        // TOFIX : Si 0 AP et WantToForfeit, il aura le WantToForfeit mais le TurnEntityCheck lui aura retiré son tour => Il l'utilisera alors au tour prochain et commencera à 0 AP : boucle sans fin.
        /* 
        info!("{:?} ne voulant rien faire, il abandonne son tour.", entity);
        commands.entity(entity).insert(WantToForfeit);
        */
        to_remove.push(entity);  
    }
    for entity in to_remove {
        commands.entity(entity).remove::<Planning>(); 
    }
}

// 0.20q : PLACEHOLDER : On place pour le moment un component Goal. Les NPC avec ce Component commenceront à planifier leurs actions.
pub fn planning_evaluate_goals(
    mut commands: Commands,
    mut entity_npc_q: Query<(Entity, Option<&mut Planning>), (With<Npc>, With<Turn>, With<CheckGoal>, Without<IsDead>)>,
    player_q: Query<(Entity, Option<&IsDead>), With<Player>>,
){
    let mut to_remove = Vec::new();

    for (_, is_dead) in player_q.iter() {
        if is_dead.is_some() {
            for (entity, _) in entity_npc_q.iter_mut() {
                commands.entity(entity).insert(WantToForfeit);
                to_remove.push(entity);
            }
        } else {        
            for (entity, planning) in entity_npc_q.iter_mut() {
                info!("Npc {:?} reflechit à ses objectifs.--------------", entity);
                match planning {
                    Some(mut has_planing) => { 
                        info!("{:?} a déjà un planning. Reset.", entity);
                        has_planing.reset(); 
                    },
                    None => { 
                        info!("{:?} n'a pas de planning. Donnons lui-en un.", entity);
                        commands.entity(entity).insert(Planning::new()); },
                };    
                to_remove.push(entity);
            }
        }
        break;  // DEGUEU, mais on est pas sensé avoir plus d'un perso et on veut pas que ca tourne plus d'une fois.
                // REMEMBEr : ca sera a refacto quand on aura plusieurs personnages....
    }
    for entity in to_remove {
        commands.entity(entity).remove::<CheckGoal>();
    }
}

// 0.20q : Est-ce que l'enemi est en vue?
pub fn planning_enemy_in_sight(
    player_position_q: Query<&BoardPosition, With<Player>>,
    mut npc_entity_fighter_q: Query<(Entity, &BoardPosition, &mut Planning), (With<Npc>, With<Turn>, Without<IsDead>)>,
    board: Res<Map>,
){
    let Ok(target_position) = player_position_q.get_single() else { return };
     for (entity, position, mut planning) in npc_entity_fighter_q.iter_mut() {
        if let Ok(_) = is_in_sight(&board, &position.v, &target_position.v, VISIBILITY_RANGE_NPC) {
            info!("Npc {:?} voit sa cible.", entity);
            planning.in_sight = true;
        } else {
            info!("Npc {:?} n'a pas de cible.", entity);
        }   
    }
}

pub fn planning_can_do_ranged_attack(
    mut npc_entity_fighter_q: Query<(Entity, &ActionPoints, &mut Planning), (With<Npc>, With<Turn>, With<Ranged>, Without<IsDead>)>, 
){
    for (entity, action_points, mut planning) in npc_entity_fighter_q.iter_mut() {
        if action_points.current >= AP_COST_RANGED {
            info!("Npc {:?} peut utiliser une attaque à distance.", entity);
            planning.ap_for_range = true;
        } else {
            info!("Npc {:?} n'a pas assez de PA pour une attaque à distance : {:?}", entity, action_points.current);
        }
    }
}

pub fn planning_adjacent_enemy(
    player_position_q: Query<&BoardPosition, With<Player>>,
    mut npc_entity_fighter_q: Query<(Entity, &BoardPosition, &mut Planning), (With<Npc>, With<Turn>, Without<IsDead>)>,
) {
    let Ok(target_position) = player_position_q.get_single() else { return };
    for (entity, position, mut planning) in npc_entity_fighter_q.iter_mut() {
        if (target_position.v.x - position.v.x).abs() < 2 && (target_position.v.y - position.v.y).abs() < 2 {
            info!("Npc {:?} est a coté de sa cible.", entity);
            planning.melee_range = true;
        } else {
            info!("Npc {:?} est éloigné de sa cible.", entity);
        }
    }
}

pub fn planning_can_do_melee_attack(
    mut npc_entity_fighter_q: Query<(Entity, &ActionPoints, &mut Planning), (With<Npc>, With<Turn>, With<Melee>, Without<IsDead>)>, 
){
    for (entity, action_points, mut planning) in npc_entity_fighter_q.iter_mut() {
        if action_points.current >= AP_COST_MELEE {
            info!("Npc {:?} peut utiliser une attaque de Melee.", entity);
            planning.ap_for_melee = true;
        } else {
            info!("Npc {:?} n'a pas assez de PA pour une attaque de Melee: {:?}.", entity, action_points.current);
        }        
    }
}

pub fn planning_has_low_life(
    mut npc_entity_fighter_q: Query<(Entity, &Health, &mut Planning), (With<Npc>, With<Turn>, Without<IsDead>)>,
) {
    for (entity, health, mut planning) in npc_entity_fighter_q.iter_mut() {
        if health.current < health.max / LOW_HP_THRESHOLD {
            info!("Npc {:?} est faible en vie", entity);
            planning.low_health = true;
        } else {
            info!("Npc {:?} estime être en bonne santé.", entity);
        }
    } 
}

pub fn planning_has_allies_nearby(
    mut npc_entity_fighter_q: Query<(Entity, &BoardPosition, &mut Planning), (With<Npc>, With<Turn>, Without<IsDead>)>,
    npc_position_q: Query<&BoardPosition, With<Npc>>,
    board: Res<Map>,
){
    for (entity, position, mut planning) in npc_entity_fighter_q.iter_mut() {
        for npc_position in npc_position_q.iter() {
            if let Ok(_) = is_in_sight(&board, &position.v, &npc_position.v, VISIBILITY_RANGE_NPC) {
                info!("Npc {:?} a des alliés proches.", entity);
                planning.has_allies_nearby = true;
                break;
            }
        }
    }
}

pub fn planning_can_move( 
    mut npc_entity_fighter_q: Query<(Entity, &ActionPoints, &mut Planning), (With<Npc>, With<Turn>, With<Walk>, Without<IsDead>)>, 
) {
    for (entity, action_points, mut planning) in npc_entity_fighter_q.iter_mut() {
        if action_points.current >= AP_COST_MOVE {
            info!("Npc {:?} peut se deplacer.", entity);
            planning.can_move = true;
        } else {
            info!("Npc {:?} n'a pas assez de PA pour se deplacer", entity);
        }        
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
        info!("Plan flee: exit found. have I a path to it?");
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
                let next_position = path.get(0).copied();
                info!("I am {:?}, i'm at {:?} and my target is {:?}", npc_entity, npc_position.v, next_position);
                //println!("NPC {:?} J'ai planifié un chemin pour moi.", npc_entity);
                info!("Plan flee: path to exit found.");
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
    npc_entity_fighter_q: Query<(Entity, &BoardPosition, &PlanSearch), (With<Npc>, With<Turn>, Without<IsDead>, With<Walk>)>,   
    board: Res<Map>,
    query_occupied: Query<&BoardPosition, With<Occupier>>,
    exit_position_q: Query<&BoardPosition, With<NavigationNode>>,
) {
    let mut to_remove_plan_move = Vec::new();
    for (npc_entity, npc_position, _) in npc_entity_fighter_q.iter() {
        info!("Plan flee: exit found. have I a path to it?");
        to_remove_plan_move.push(npc_entity);        

        // Utilisé dans room_based_exits. 
        let mut nearest_distance = 0;
        let mut nearest_destination= npc_position.v;
        for room in exit_position_q.iter() {
            let distance = npc_position.v.clone().manhattan(room.v);
            if distance > nearest_distance {
                nearest_distance = distance;
                nearest_destination = room.v; 
            }
        }

        // Similaire à Flee et Exit de MapBuilder.
        if nearest_destination != npc_position.v {
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
            for (npc_entity, _, _) in npc_entity_fighter_q.iter() {
                commands.entity(npc_entity).insert(WantToForfeit);  // Securité pour ne pas rester bloqué.
            }
        }
    }
    for entity in to_remove_plan_move {
        commands.entity(entity).remove::<PlanSearch>();   
    }
}