use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{game::{player::Player, ui::ReloadUiEvent, rules::{consume_actionpoints, roll_dices_against}, tileboard::components::BoardPosition, pieces::components::{Occupier, Piece, Stats, Health}, combat::{AP_COST_MOVE, AP_COST_MELEE}}, map_builders::map::Map, vectors::{find_path, Vector2Int}, render::{get_final_world_position, components::PathAnimator}};

use super::{events::{EntityEndTurnEvent, Turn, EntityTryMoveEvent, EntityMoveEvent, AnimateEvent, OnClickEvent, EntityHitTryEvent, EntityGetHitEvent, EntityDeathEvent}, components::ActionPoints};


/// Gestion de l'action de forfeit.
pub fn action_entity_end_turn(
    mut ev_endturn: EventReader<EntityEndTurnEvent>,
    mut query_character_turn: Query<(Entity, &mut ActionPoints, Option<&Player>), With<Turn>>,
    mut ev_interface: EventWriter<ReloadUiEvent>,  
) {
    //println!("action entity forfeit turn");
    for event in ev_endturn.iter() {
        //L'entité n'a pas de Action points / Pas son tour, on ignore.
        let Ok(entity_infos) = query_character_turn.get_mut(event.entity) else { continue };
        let (_entity, mut action_points, is_player) = entity_infos;

        let lost_value = action_points.max.saturating_add(0);
        consume_actionpoints(&mut action_points, lost_value);
        
        if is_player.is_some() {
            ev_interface.send(ReloadUiEvent);
        }
    }
}

/// Player clicked on a tile.
pub fn on_click_action(
    mut ev_onclick: EventReader<OnClickEvent>,
    mut ev_try_move: EventWriter<EntityTryMoveEvent>,
    piece_position: Query<(Entity, &BoardPosition), With<Piece>>,
    mut query_character_turn: Query<(&BoardPosition, Option<&Player>), With<Turn>>,
    query_occupied: Query<&BoardPosition, With<Occupier>>,
    board: Res<Map>,
){
    for event in ev_onclick.iter() {
        println!("Click for entity : {:?}, with tile {:?}", event.entity, event.tile);
        if !board.entity_tiles.contains_key(&event.tile) { return };    //Hors map.

        let mut has_target = false;
        if piece_position.iter().any(|(_entity, board_position)| board_position.v == event.tile) {
            has_target = true;
        }

        if has_target {
            println!("J'ai une cible!");
        }

        let Ok(entity_infos) = query_character_turn.get_mut(event.entity) else { 
            println!("Caracter info querry None");
            return };
        let (position, _is_player) = entity_infos;


        let path_to_destination = find_path(
            position.v,
            event.tile,
            &board.entity_tiles.keys().cloned().collect(),
            &query_occupied.iter().map(|p| p.v).collect(),
            has_target,
        ); 

        let Some(path) = path_to_destination else { 
            println!("Pas de Path");
            return };

        let pathing = path.clone();
        let mut target = None;
        if has_target {
            target = Some(event.tile);
        }
        ev_try_move.send(EntityTryMoveEvent {entity: event.entity, path: pathing, target});
    }
}


/// Test de l'action Move.
pub fn action_entity_try_move(
    //mut query_character_turn: Query<(&ActionPoints, &BoardPosition, Option<&Player>), With<Turn>>,
    //query_occupied: Query<&BoardPosition, With<Occupier>>,
    //board: Res<Map>,
    mut ev_try_move: EventReader<EntityTryMoveEvent>,
    mut ev_move: EventWriter<EntityMoveEvent>,
    mut ev_try_attack: EventWriter<EntityHitTryEvent>,
    query_actions: Query<&ActionPoints>,
){
    for event in ev_try_move.iter() {
        let Ok(action_points) = query_actions.get(event.entity) else { continue };
        if action_points.current < AP_COST_MOVE { continue };

        // Target check
        let destination = event.path.get(0);
        let Some(new_position) = destination else { continue };
        if let Some(current_target) = event.target {
            if &current_target == new_position {
                if action_points.current < AP_COST_MELEE { continue };
                println!("J'attaque ma cible!!!");
                ev_try_attack.send( EntityHitTryEvent {entity: event.entity, target: current_target});
                continue
            }
        }        

        let path = event.path.clone();
        ev_move.send(EntityMoveEvent {entity: event.entity, path: path, target: event.target});

        /* 
        println!("action entity try move: {:?}", event.entity);
        let Ok(entity_infos) = query_character_turn.get_mut(event.entity) else { 
            println!("Caracter info querry None");
            return };
        let (action_points, position, _is_player) = entity_infos;

        let path_to_destination = find_path(
            position.v,
            event.destination,
            &board.entity_tiles.keys().cloned().collect(),
            &query_occupied.iter().map(|p| p.v).collect()
        ); 

        let Some(path) = path_to_destination else { 
            println!("Pas de Path");
            return };
        let ap_cost = path.len() as u32;
        if action_points.current < ap_cost { 
            println!("Pas d'action points");
            return };

        let pathing = path.clone();

        ev_move.send(EntityMoveEvent {entity: event.entity, path: pathing});
        */
    }
}


pub fn action_entity_try_attack(
    mut ev_try_attack: EventReader<EntityHitTryEvent>,    
    mut ev_gethit: EventWriter<EntityGetHitEvent>,
    //position_q: Query<&BoardPosition>,
    available_targets: Query<(Entity, &BoardPosition, &Stats), With<Health>>,
    stats_q: Query<&Stats>,
){
    for event in ev_try_attack.iter() {
        println!("Je suis {:?} et j'attaque {:?}", event.entity, event.target);

        /*  
        let Ok(attacker_position) = position_q.get(event.entity) else { 
            println!("Pas de position pour l'attaquant");
            continue };

        //NOT: We dont do this check because: Manhattan is > 1 for Diagonal & we should be melee already when this check happens.
        if attacker_position.v.manhattan(event.target) > 1 { 
            println!("Cible hors de portée.");
            continue };
        */
        // Targets de la case:

        let target_entities = available_targets.iter().filter(|(_, position,_)| position.v == event.target).collect::<Vec<_>>(); 
        if target_entities.len() == 0 { 
            println!("Pas de cible ici.");
            continue }; 

        let Ok(attacker_stats) = stats_q.get(event.entity) else { 
            println!("Pas de stats pour l'attaquant");
            continue };        
        for (target_entity, _target_position, target_stats) in target_entities.iter() {     
            // Can't hit yourself.
            if event.entity == * target_entity { 
                println!("On ne peut pas s'attaquer soit même.");
                continue; } 

            let dice_roll = roll_dices_against(attacker_stats.attack, target_stats.dodge);   
            let dmg = dice_roll.success.saturating_add(attacker_stats.power as u32);
            if dice_roll.success > 0 {
                println!("HIT target with {:?} success! for {:?} dmg", dice_roll.success, dmg);
                ev_gethit.send(EntityGetHitEvent { entity: * target_entity, attacker: event.entity, dmg: dmg })
            } else {
                println!("Miss target.");
            }
        }
    }
}


pub fn action_entity_get_hit(
    mut ev_gethit: EventReader<EntityGetHitEvent>,
    mut stats_health_q: Query<(&Stats, &mut Health, Option<&Player>)>,
    mut ev_die: EventWriter<EntityDeathEvent>,

) {
    for event in ev_gethit.iter() {
        println!("Entity {:?} has been hit by {:?} for {:?} dmg.", event.entity, event.attacker, event.dmg);
        let Ok(defender_infos) = stats_health_q.get_mut(event.entity) else { 
            println!("Pas de stats / health pour le defender");
            return };
        let (defender_stats, mut defender_health, is_player) = defender_infos;

        // Roll resist.
        let dice_roll = roll_dices_against(defender_stats.resilience, 0);       // Pas d'opposant ni difficulté : On encaisse X dmg.
        let dmg = event.dmg.saturating_sub(dice_roll.success); 

        // Reducing health.
        defender_health.current = defender_health.current.saturating_sub(dmg);
        println!("Dmg on health for {:?} is now {:?}/{:?}", dmg, defender_health.current, defender_health.max);
        if defender_health.current == 0 {
            if let Some(_is_player) = is_player {
                println!("GAME OVER !");    //TODO
            } else {
                ev_die.send(EntityDeathEvent { entity: event.entity })
            }
        }
    }
}

pub fn entity_dies(
    mut ev_die: EventReader<EntityDeathEvent>,
){
    for event in ev_die.iter() {
        println!("Entity {:?} is dead", event.entity);
        //TODO : Retirer les Composants Health and Co.
        //TODO : Transformer en Corps.
    }
}

/// Gestion de l'action Move.
pub fn action_entity_move(
    mut query_character_turn: Query<(Entity, &mut ActionPoints, &mut BoardPosition, Option<&Player>), With<Turn>>,
    mut ev_move: EventReader<EntityMoveEvent>,
    mut ev_interface: EventWriter<ReloadUiEvent>,
    mut ev_animate: EventWriter<AnimateEvent>,
    mut ev_try_move: EventWriter<EntityTryMoveEvent>,
){    
    for event in ev_move.iter() {
        let Ok(entity_infos) = query_character_turn.get_mut(event.entity) else { 
            //println!("ActionMove: Je n'ai pas les infos Entité");   // TODO : Quand Action_entity_try_move pose le component MovePath, le Query action_entity_move ne le recupere pas pour le moment (asynchrone?)
            continue };
        let (entity, mut action_points,mut board_position, is_player) = entity_infos;  

        let mut path = event.path.clone();
        let destination = path.pop_front();
        let Some(new_position) = destination.clone() else { break };
        
        board_position.v = new_position;
        ev_try_move.send(EntityTryMoveEvent {entity: event.entity, path: path, target: event.target});

        action_points.current = action_points.current.saturating_sub(AP_COST_MOVE);
        if is_player.is_some() {
            ev_interface.send(ReloadUiEvent);
        }

        let mut path_animation: VecDeque<Vector2Int> = VecDeque::new();
        path_animation.push_back(new_position);
        ev_animate.send(AnimateEvent { entity: entity, path: path_animation });
    }
    /* 
    for event in ev_move.iter() {
        println!("action entity move");        
        let Ok(entity_infos) = query_character_turn.get_mut(event.entity) else { 
            println!("ActionMove: Je n'ai pas les infos Entité");   // TODO : Quand Action_entity_try_move pose le component MovePath, le Query action_entity_move ne le recupere pas pour le moment (asynchrone?)
            continue };
        let (entity, mut action_points,mut board_position, is_player) = entity_infos;

        let mut path_animation: VecDeque<Vector2Int> = VecDeque::new();
        let mut path = event.path.clone();
        while !path.is_empty() {
            let destination = path.pop_front();
            let Some(new_position) = destination else { 
                println!("Je n'ai pas de nouvelle position à faire");
                break };    // Normalement, il y a tjrs qq chose.
            board_position.v = new_position;
            action_points.current = action_points.current.saturating_sub(AP_COST_MOVE);
            path_animation.push_back(new_position);

            if is_player.is_some() {
                ev_interface.send(ReloadUiEvent);
            }
        }
        ev_animate.send(AnimateEvent {entity: entity, path: path_animation});
        // On supprime à la fin.
        //commands.entity(entity).remove::<MovePath>();
        //println!("Entity {:?} has MovePath removed.", entity);
        
        //TODO : anim
        //commands.entity(entity).insert(PathAnimator{path:VecDeque::from([target]), wait_anim: false});
    }
    */
}

pub fn walk_combat_animation(    
    mut commands: Commands,
    mut ev_animate: EventReader<AnimateEvent>,
    query_piece: Query<&Piece>,
) {
    for ev in ev_animate.iter() {
        let Ok(piece) = query_piece.get(ev.entity) else { continue };
        let mut path = ev.path.clone();

        let mut path_animation: VecDeque<Vec3> = VecDeque::new();
        while !ev.path.is_empty() {
            let step = path.pop_front();
            let Some(current_step) = step else { break };
            let target = get_final_world_position(current_step, piece.size);
            path_animation.push_back(target);
        }
        println!("PathAnimator created");
        commands.entity(ev.entity).insert(PathAnimator{path:VecDeque::from(path_animation), wait_anim: true});        
    }
}

pub struct ActionMoveToTarget {
    pub cost: Option<u32>,
    pub path: Option<VecDeque<Vector2Int>>,
    pub target: Option<Vector2Int>
}

pub fn get_ap_cost(
    mut query_character_turn: Query<(&ActionPoints, &BoardPosition)>,
    query_occupied: Query<&BoardPosition, With<Occupier>>,
    board: Res<Map>,
    tile_position: Vector2Int,
    entity: Entity,
) -> Option<u32> {
    let mut result = ActionMoveToTarget { cost: None, path: None, target: None};

    let Ok(entity_infos) = query_character_turn.get_mut(entity) else { 
        println!("Caracter info querry None");
    return result.cost};
    
    let (action_points, position) = entity_infos;

    let path_to_destination = find_path(
        position.v,
        tile_position,
        &board.entity_tiles.keys().cloned().collect(),
        &query_occupied.iter().map(|p| p.v).collect(),
        true,
    ); 

    let Some(path) = path_to_destination else { 
        println!("Pas de Path");
    return result.cost};

    let ap_cost = path.len() as u32;
    
    if action_points.current >= ap_cost {
        result.cost = Some(ap_cost);
    };
    result.cost
}