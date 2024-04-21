use bevy::prelude::*;

use crate::{game::{
    combat::{combat_system::components::{ActionPoints, AttackType, IsDead, WantToForfeit, WantToHit},
    events::Turn, 
    rules::{AP_COST_MELEE, AP_COST_MOVE, AP_COST_RANGED, LOW_HP_THRESHOLD, VISIBILITY_RANGE_NPC}}, 
    commons::is_in_sight, 
    pieces::components::{Health, Melee, Npc, Ranged, Walk}, player::Player, tileboard::components::BoardPosition},
    map_builders::map::Map
};

use super::components::CheckGoal;


#[derive(Component)]
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
}


// 0.20q : PLACEHOLDER : On place pour le moment un component Goal. Les NPC avec ce Component commenceront à planifier leurs actions.
pub fn planning_evaluate_goals(
    mut commands: Commands,
    entity_npc_q: Query<Entity, (With<Npc>, With<Turn>, With<CheckGoal>, Without<IsDead>)>,
){
    for entity in entity_npc_q.iter() {
        info!("Npc {:?} reflechit à ses objectifs.--------------", entity);
        commands.entity(entity).insert(Planning::new());
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
            info!("Npc {:?} n'a pas assez de PA pour une attaque à distance.", entity);
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
            info!("Npc {:?} n'a pas assez de PA pour une attaque de Melee.", entity);
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
            } 
            // Si je ne peux pas, qu'est ce que je fais?
            if planning.can_move {
                info!("{:?} va s'éloigner", entity);
                //continue
            }          
        } 
        if planning.can_move {
            info!("{:?} va se deplacer au hasard pour chercher sa cible.", entity);
            //continue 
        }
        info!("{:?} ne voulant rien faire, il abandonne son tour.", entity);
        commands.entity(entity).insert(WantToForfeit);
        to_remove.push(entity);  
    }
    for entity in to_remove {
        commands.entity(entity).remove::<Planning>(); 
    }
}

