use bevy::prelude::*;

use crate::{game::{
    combat::{combat_system::components::{ActionPoints, AttackType, IsDead, WantToForfeit, WantToHit}, 
    events::Turn, ia::components::{PlanFlee, PlanMove, PlanSearch}, 
    rules::{AP_COST_MELEE, AP_COST_MOVE, AP_COST_RANGED, LOW_HP_THRESHOLD, VISIBILITY_RANGE_NPC}}, commons::is_in_sight, game_generation::character_creation::components::{Health, Melee, Npc, Ranged, Walk}, movements::components::HasMoved, player::Player, tileboard::components::BoardPosition}, map_builders::map::Map
};

use super::{components::{CheckGoal, HasShareInfos, Knowledge, PlanInformAllies}, Planning};

 

// 0.20r : v1.1 : Les protections si on ne peut pas bouger sont dans les PlanMove pour le moment. C'est pas ouf.
// 0.20q : v1 : NOTE : On est très vulnerable à un problème entre l'estimation et la résolution, qui bloquera le jeu.
// TODO : Prevoir les actions, les jouer une à une jusqu'à ce qu'il n'y en ai plus.
pub fn planning_actions(
    mut commands: Commands,
    npc_entity_fighter_q: Query<(Entity, &Planning, &Knowledge), (With<Npc>, With<Turn>, Without<IsDead>)>,
    entity_player_q: Query<Entity, With<Player>>,
    position_q: Query<&BoardPosition>,

) {     
    let Ok(target) = entity_player_q.get_single() else { return };
    let Ok(target_position) = position_q.get(target) else { return };
    let mut to_remove = Vec::new();

    for (entity, planning, knowledge) in npc_entity_fighter_q.iter() {
        //info!!("{:?} is planning -----------------", entity);
        //info!!("{:?}", planning);
        
                // Attaque à distance
        if planning.in_sight && planning.ap_for_range {
            //info!!("{:?} va attaquer sa cible à distance.", entity);
            commands.entity(entity).insert(WantToHit { mode: AttackType::RANGED, target: target_position.v });
        }
        // Melee 
        if planning.in_sight && planning.ap_for_melee && planning.melee_range {
            //info!!("{:?} va attaquer sa cible en melee.", entity);
            commands.entity(entity).insert(WantToHit { mode: AttackType::MELEE, target: target_position.v });
        }
        // Trop loin de la cible mais peut taper.
        if planning.in_sight && planning.ap_for_melee && planning.can_move {
            // TODO : Doit verifier s'il peut porter un coup ce tour.
            //info!!("{:?} va se rapprocher de sa cible pour l'attaquer en melee!", entity);
            commands.entity(entity).insert(PlanMove { destination: target_position.v}); 
        }
        // Sait / pense savoir ou il est, && a des alliés proches: les informe, sauf si deja fait.
        if planning.has_allies_nearby && planning.know_target_position && !planning.has_shared_infos {
            if let Some(known_position) = knowledge.player_last_seen {
                commands.entity(entity).insert(PlanInformAllies { target_position: known_position });
            }            
        }

        // En vue, mais ne peut pas taper.
        if planning.in_sight && !planning.ap_for_melee && !planning.ap_for_range && planning.can_move {
            //s'eloigne
            //info!!("{:?} va s'éloigner", entity);
            commands.entity(entity).insert(PlanFlee { away_from: target_position.v}); 
        }
        // Connait une position possible de sa cible, et s'y rends.
        if planning.know_target_position && planning.can_move {
            commands.entity(entity).insert(PlanMove { destination: knowledge.player_last_seen.expect("Checked in known_target_position") }); 
        }
        // Ne voit pas la cible : la cherche.
        if !planning.in_sight && planning.can_move {
            //info!!("{:?} recherche sa cible.", entity);
            commands.entity(entity).insert(PlanSearch ); 
        } 
        to_remove.push(entity);  
    }
    for entity in to_remove {
        commands.entity(entity).remove::<Planning>(); 
    }
}


// 0.20q : PLACEHOLDER : On place pour le moment un component Goal. Les NPC avec ce Component commenceront à planifier leurs actions.
pub fn ia_evaluate_goals(
    mut commands: Commands,
    mut entity_npc_q: Query<(Entity, Option<&mut Planning>, Option<&mut Knowledge>), (With<Npc>, With<Turn>, With<CheckGoal>, Without<IsDead>)>,
    player_q: Query<(Entity, Option<&IsDead>), With<Player>>,
){
    let mut to_remove = Vec::new();

    for (_, is_dead) in player_q.iter() {
        if is_dead.is_some() {
            for (entity, _, _) in entity_npc_q.iter_mut() {
                commands.entity(entity).insert(WantToForfeit);
                to_remove.push(entity);
            }
        } else {        
            for (entity, planning, knowledge) in entity_npc_q.iter_mut() {
                //info!!("Npc {:?} reflechit à ses objectifs.--------------", entity);
                match planning {
                    Some(mut has_planing) => { 
                        //info!!("{:?} a déjà un planning. Reset.", entity);
                        has_planing.reset(); 
                    },
                    None => { 
                        //info!!("{:?} n'a pas de planning. Donnons lui-en un.", entity);
                        commands.entity(entity).insert(Planning::new()); },
                };
                match knowledge {
                    Some(_has_knowledge) => { },
                    None => {
                        commands.entity(entity).insert(Knowledge { player_last_seen: None, last_visited_nodes: Vec::new() });
                    }
                }
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

// 0.20t : On regarde s'il a deja share Knowledge, pour ne pas rester bloquer. HasShareInfos est retiré au debut du tour de l'Entité. TODO : Pas fou.
pub fn ia_has_shared_knowledge(
    mut npc_entity_fighter_q: Query<&mut Planning, (With<Npc>, With<Turn>, Without<IsDead>, With<HasShareInfos>)>,
) {
    for mut planning in npc_entity_fighter_q.iter_mut() {
        planning.has_shared_infos = true;
    }
}

// 0.20r : Est ce que je vois tjrs la dernière position connue de ma cible?
pub fn ia_evaluate_check_target_knowledge(
    mut commands: Commands,
    player_position_q: Query<&BoardPosition, With<Player>>,
    player_moved_q: Query<&HasMoved, With<Player>>,
    player_entity_q: Query<Entity, With<Player>>,
    mut npc_entity_fighter_q: Query<(Entity, &BoardPosition, &mut Planning, &mut Knowledge), (With<Npc>, With<Turn>, Without<IsDead>)>,
    board: Res<Map>,
){
    let Ok (player_entity) = player_entity_q.get_single() else { return };
    let Ok (target_position) = player_position_q.get(player_entity) else { return };
    for (entity, position,  mut planning, mut knowledge) in npc_entity_fighter_q.iter_mut() {
        match knowledge.player_last_seen {
            None => { 
                planning.know_target_position = false;                
                commands.entity(entity).remove::<HasShareInfos>();  // 0.20t : necessaire pour qu'il puisse partager de nouveau info.
                //info!("N'a pas vu le PJ recemment.");
                continue },
            Some(last_known_position) => {
                if let Ok(_) = is_in_sight(&board, &position.v, &last_known_position, VISIBILITY_RANGE_NPC) {
                    //info!("Je vois l'endroit où est ma cible.");
                    if target_position.v != last_known_position {
                        //info!("Ma cible n'est pas là où je le pensais.");
                        // Je regarde si je l'ai vu se deplacer.
                        knowledge.player_last_seen = None;
                        planning.know_target_position = false;                        
                        commands.entity(entity).remove::<HasShareInfos>();  // 0.20t : necessaire pour qu'il puisse partager de nouveau info quand il l'aura.
                        if let Ok(has_moved) = player_moved_q.get(player_entity) {
                            //info!("Il s'est déplacé! Est ce que je l'ai vu faire?");                         
                            // 0.20t : Ajout de Moved avec le contenu du trajet fait après un deplacement. On regarde si on a vu le PJ se deplacer. Ca reste bof.
                            for &visited_tile in &has_moved.visited_tiles[..] {
                                if let Ok(_) = is_in_sight(&board, &position.v, &visited_tile, VISIBILITY_RANGE_NPC) {
                                    //info!("Mais je l'ai vu partir par là!");
                                    knowledge.player_last_seen = Some(visited_tile.clone());
                                    planning.know_target_position = true;
                                    break;
                                }
                            }
                        }
                    } else {
                        planning.know_target_position = true;
                    }
                }
            }
        };
    }
}

// 0.20q : Est-ce que l'enemi est en vue?
pub fn ia_evaluate_enemy_in_sight(
    player_position_q: Query<&BoardPosition, With<Player>>,
    mut npc_entity_fighter_q: Query<(Entity, &BoardPosition, &mut Planning, &mut Knowledge), (With<Npc>, With<Turn>, Without<IsDead>)>,
    board: Res<Map>,
){
    let Ok(target_position) = player_position_q.get_single() else { return };
     for (_entity, position, mut planning, mut knowledge) in npc_entity_fighter_q.iter_mut() {
        if let Ok(_) = is_in_sight(&board, &position.v, &target_position.v, VISIBILITY_RANGE_NPC) {
            //info!!("Npc {:?} voit sa cible.", entity);
            planning.in_sight = true;
            knowledge.player_last_seen = Some(target_position.v.clone());
        } else {
            //info!!("Npc {:?} n'a pas de cible.", entity);
        }   
    }
}

// 0.20r 
pub fn ia_evaluate_know_target_position(
    mut npc_entity_fighter_q: Query<(&mut Planning, &mut Knowledge), (With<Npc>, With<Turn>, Without<IsDead>)>,
){
    for (mut planning, knowledge) in npc_entity_fighter_q.iter_mut() {
        match knowledge.player_last_seen {
            Some(_) => {planning.know_target_position= true;},
            None => {}
        }
    }
}

pub fn ia_evaluate_can_do_ranged_attack(
    mut npc_entity_fighter_q: Query<(Entity, &ActionPoints, &mut Planning), (With<Npc>, With<Turn>, With<Ranged>, Without<IsDead>)>, 
){
    for (_entity, action_points, mut planning) in npc_entity_fighter_q.iter_mut() {
        if action_points.current >= AP_COST_RANGED {
            //info!!("Npc {:?} peut utiliser une attaque à distance.", entity);
            planning.ap_for_range = true;
        } else {
            //info!!("Npc {:?} n'a pas assez de PA pour une attaque à distance : {:?}", entity, action_points.current);
        }
    }
}

pub fn ia_evaluate_adjacent_enemy(
    player_position_q: Query<&BoardPosition, With<Player>>,
    mut npc_entity_fighter_q: Query<(Entity, &BoardPosition, &mut Planning), (With<Npc>, With<Turn>, Without<IsDead>)>,
) {
    let Ok(target_position) = player_position_q.get_single() else { return };
    for (_entity, position, mut planning) in npc_entity_fighter_q.iter_mut() {
        if (target_position.v.x - position.v.x).abs() < 2 && (target_position.v.y - position.v.y).abs() < 2 {
            //info!!("Npc {:?} est a coté de sa cible.", entity);
            planning.melee_range = true;
        } else {
            //info!!("Npc {:?} est éloigné de sa cible.", entity);
        }
    }
}

pub fn ia_evaluate_can_do_melee_attack(
    mut npc_entity_fighter_q: Query<(Entity, &ActionPoints, &mut Planning), (With<Npc>, With<Turn>, With<Melee>, Without<IsDead>)>, 
){
    for (_entity, action_points, mut planning) in npc_entity_fighter_q.iter_mut() {
        if action_points.current >= AP_COST_MELEE {
            //info!!("Npc {:?} peut utiliser une attaque de Melee.", entity);
            planning.ap_for_melee = true;
        } else {
            //info!!("Npc {:?} n'a pas assez de PA pour une attaque de Melee: {:?}.", entity, action_points.current);
        }        
    }
}

pub fn ia_evaluate_has_low_life(
    mut npc_entity_fighter_q: Query<(Entity, &Health, &mut Planning), (With<Npc>, With<Turn>, Without<IsDead>)>,
) {
    for (_entity, health, mut planning) in npc_entity_fighter_q.iter_mut() {
        if health.current < health.max / LOW_HP_THRESHOLD {
            //info!!("Npc {:?} est faible en vie", entity);
            planning.low_health = true;
        } else {
            //info!!("Npc {:?} estime être en bonne santé.", entity);
        }
    } 
}

pub fn ia_evaluate_allies_nearby(
    mut npc_entity_fighter_q: Query<(Entity, &BoardPosition, &mut Planning), (With<Npc>, With<Turn>, Without<IsDead>)>,
    npc_position_q: Query<&BoardPosition, With<Npc>>,
    board: Res<Map>,
){
    for (_entity, position, mut planning) in npc_entity_fighter_q.iter_mut() {
        for npc_position in npc_position_q.iter() {
            // TODO : Enregistrer les alliés proches?
            if let Ok(_) = is_in_sight(&board, &position.v, &npc_position.v, VISIBILITY_RANGE_NPC) {
                //info!!("Npc {:?} a des alliés proches.", entity);
                planning.has_allies_nearby = true;
                break;
            }
        }
    }
}

pub fn ia_evaluate_can_move( 
    mut npc_entity_fighter_q: Query<(Entity, &ActionPoints, &mut Planning), (With<Npc>, With<Turn>, With<Walk>, Without<IsDead>)>, 
) {
    for (_entity, action_points, mut planning) in npc_entity_fighter_q.iter_mut() {
        if action_points.current >= AP_COST_MOVE {
            //info!!("Npc {:?} peut se deplacer.", entity);
            planning.can_move = true;
        } else {
            //info!!("Npc {:?} n'a pas assez de PA pour se deplacer", entity);
        }        
    }
}
