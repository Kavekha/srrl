use bevy::prelude::*;

use crate::game::{combat::{combat_system::components::{ActionPoints, IsDead, WantToForfeit}, events::Turn, ia::components::Goal}, pieces::components::{Health, Melee, Npc, Ranged, Walk}, tileboard::components::BoardPosition};

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
        info!("Npc {:?} reflechit à ses objectifs.", entity);
        commands.entity(entity).insert(Planning::new());
    }
}

// 0.20q : Est-ce que l'enemi est en vue?
pub fn planning_enemy_in_sight(
    mut commands: Commands,
    mut npc_entity_fighter_q: Query<(Entity, &BoardPosition, &mut Planning), (With<Npc>, With<Turn>, Without<IsDead>)>,
){
    for (entity, position, mut planning) in npc_entity_fighter_q.iter_mut() {
        info!("Npc {:?} regarde s'il a un enemi en vue.", entity);
        // Oui.
        planning.in_sight = true;
    }
}

pub fn planning_can_do_ranged_attack(
    mut npc_entity_fighter_q: Query<(Entity, &ActionPoints, &mut Planning), (With<Npc>, With<Turn>, With<Ranged>, Without<IsDead>)>, 
){
    for (entity, action_points, mut planning) in npc_entity_fighter_q.iter_mut() {
        info!("Npc {:?} regarde s'il peut attaquer à distance.", entity);
        // Oui
        planning.ap_for_range = true;
    }
}

pub fn planning_adjacent_enemy(
    mut npc_entity_fighter_q: Query<(Entity, &BoardPosition, &mut Planning), (With<Npc>, With<Turn>, Without<IsDead>)>,
) {
    for (entity, position, mut planning) in npc_entity_fighter_q.iter_mut() {
        info!("Npc {:?} est a coté de l'ennemi?", entity);
        // oui 
        planning.melee_range = true;
    }
}

pub fn planning_can_do_melee_attack(
    mut npc_entity_fighter_q: Query<(Entity, &ActionPoints, &mut Planning), (With<Npc>, With<Turn>, With<Melee>, Without<IsDead>)>, 
){
    for (entity, action_points, mut planning) in npc_entity_fighter_q.iter_mut() {
        info!("Npc {:?} regarde s'il peut attaquer en melee.", entity);
        // Oui
        planning.ap_for_melee = true;
    }
}

pub fn planning_has_low_life(
    mut npc_entity_fighter_q: Query<(Entity, &Health, &mut Planning), (With<Npc>, With<Turn>, Without<IsDead>)>,
) {
    for (entity, health, mut planning) in npc_entity_fighter_q.iter_mut() {
        info!("Npc {:?} regarde s'il est faible en vie", entity);
        // Oui
        planning.low_health = true;
    } 
}

pub fn planning_has_allies_nearby(
    mut npc_entity_fighter_q: Query<(Entity, &BoardPosition, &mut Planning), (With<Npc>, With<Turn>, Without<IsDead>)>,
){
    for (entity, position, mut planning) in npc_entity_fighter_q.iter_mut() {
        info!("Npc {:?} a des alliés proches.", entity);
        // Oui 
        planning.has_allies_nearby = true;
    }
}

pub fn planning_can_move( 
    mut npc_entity_fighter_q: Query<(Entity, &ActionPoints, &mut Planning), (With<Npc>, With<Turn>, With<Walk>, Without<IsDead>)>, 
) {
    for (entity, action_points, mut planning) in npc_entity_fighter_q.iter_mut() {
        info!("Npc {:?} peut bouger.", entity);
    }
}


pub fn npc_ai_plan_forfeit(
    mut commands: Commands,
    npc_entity_fighter_q: Query<(Entity, &ActionPoints), (With<Npc>, With<Turn>, With<Planning>, Without<IsDead>)>,
) {
    let mut to_remove = Vec::new();
    for (npc_entity, _) in npc_entity_fighter_q.iter() {
        to_remove.push(npc_entity);
        //println!("NPC {:?} n'a rien a faire.", npc_entity);
        commands.entity(npc_entity).insert(WantToForfeit);
    }
    for entity in to_remove {
        commands.entity(entity).remove::<Planning>();
    }
}