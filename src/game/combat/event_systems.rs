use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{
    engine::{
        audios::SoundEvent, render::{components::PathAnimator, get_world_position}
    },
    game::{
        combat::{AP_COST_MELEE, AP_COST_MOVE}, gamelog::LogEvent, manager::{game_messages::GameOverMessage, MessageEvent}, pieces::components::{Health, Occupier, Stats}, player::{Cursor, Player}, rules::{consume_actionpoints, roll_dices_against}, tileboard::components::BoardPosition, ui::ReloadUiEvent}, 
        map_builders::map::Map, 
        vectors::{find_path, Vector2Int}
    };

use super::{
    events::{EntityEndTurnEvent, Turn, EntityTryMoveEvent, EntityMoveEvent, AnimateEvent, OnClickEvent, EntityHitTryEvent, EntityGetHitEvent, EntityDeathEvent, RefreshActionCostEvent},
    components::ActionPoints
};


/// Gestion de l'action de forfeit.
pub fn action_entity_end_turn(
    mut ev_endturn: EventReader<EntityEndTurnEvent>,
    mut query_character_turn: Query<(Entity, &mut ActionPoints, Option<&Player>), With<Turn>>,
    mut ev_interface: EventWriter<ReloadUiEvent>,  
    mut ev_refresh_action: EventWriter<RefreshActionCostEvent>,
) {
    //println!("action entity forfeit turn");
    for event in ev_endturn.read() {
        //L'entité n'a pas de Action points / Pas son tour, on ignore.
        let Ok(entity_infos) = query_character_turn.get_mut(event.entity) else { continue };
        let (_entity, mut action_points, is_player) = entity_infos;

        let lost_value = action_points.max.saturating_add(0);
        consume_actionpoints(&mut action_points, lost_value);
        
        if is_player.is_some() {
            ev_interface.send(ReloadUiEvent);
            ev_refresh_action.send(RefreshActionCostEvent);
        }
    }
}

/// Player clicked on a tile.
pub fn on_click_action(
    mut ev_onclick: EventReader<OnClickEvent>,
    mut ev_try_move: EventWriter<EntityTryMoveEvent>,
    action_infos: Res<ActionInfos>,
){
    for _event in ev_onclick.read() {
        let path = action_infos.path.clone();
        let Some(entity) = action_infos.entity else { continue };
        let Some(path) = path else { continue };

        println!("On clic action: OK. Send event.");
        ev_try_move.send(EntityTryMoveEvent {entity: entity, path: path, target: action_infos.target });

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


pub fn action_entity_try_attack(
    mut ev_try_attack: EventReader<EntityHitTryEvent>,    
    mut ev_gethit: EventWriter<EntityGetHitEvent>,
    mut action_q: Query<&mut ActionPoints>,
    position_q: Query<&BoardPosition>,
    player_q: Query<&Player>,
    available_targets: Query<(Entity, &BoardPosition, &Stats), With<Health>>,
    stats_q: Query<&Stats>,
    mut ev_interface: EventWriter<ReloadUiEvent>,    
    mut ev_refresh_action: EventWriter<RefreshActionCostEvent>,
    mut ev_animate: EventWriter<AnimateEvent>,
    mut ev_sound: EventWriter<SoundEvent>,
    mut ev_log: EventWriter<LogEvent>
){
    for event in ev_try_attack.read() {
        println!("Je suis {:?} et j'attaque {:?}", event.entity, event.target);

        // TODO : maybe refresh in consume? Maybe consume in some Event?
        let Ok(mut action_points) = action_q.get_mut(event.entity) else { continue };
        consume_actionpoints(&mut action_points, AP_COST_MELEE);
        if let Ok(_is_player) = player_q.get(event.entity) {
        //if is_player.is_some() {
            ev_interface.send(ReloadUiEvent);
        }

        // Targets de la case:
        let target_entities = available_targets.iter().filter(|(_, position,_)| position.v == event.target).collect::<Vec<_>>(); 
        if target_entities.len() == 0 { 
            // DEBUG: println!("Pas de cible ici.");
            continue }; 

        let Ok(attacker_stats) = stats_q.get(event.entity) else { 
            // DEBUG: println!("Pas de stats pour l'attaquant");
            continue };        
        for (target_entity, target_position, target_stats) in target_entities.iter() {     
            // Can't hit yourself.
            if event.entity == * target_entity { 
                println!("On ne peut pas s'attaquer soit même.");
                continue; } 

            let dice_roll = roll_dices_against(attacker_stats.attack, target_stats.dodge);   
            let dmg = dice_roll.success.saturating_add(attacker_stats.power as u32);
            if dice_roll.success > 0 {
                // DEBUG: println!("HIT target with {:?} success! for {:?} dmg", dice_roll.success, dmg);
                ev_gethit.send(EntityGetHitEvent { entity: * target_entity, attacker: event.entity, dmg: dmg });
                // SOUND
                ev_sound.send(SoundEvent{id:"hit_punch_1".to_string()});
                ev_log.send(LogEvent {entry: format!("{:?} hit {:?} for {:?} damages.", event.entity, target_entity, dmg)});        // Log v0
            } else {
                // DEBUG: println!("Miss target.");
                // SOUND
                ev_sound.send(SoundEvent{id:"hit_air_1".to_string()});
                ev_log.send(LogEvent {entry: format!("{:?} try to hit {:?} but misses.", event.entity, target_entity)});        // Log v0
            }

            // Animation.
            if let Ok(entity_position) = position_q.get(event.entity) {
                let mut path_animation: VecDeque<Vector2Int> = VecDeque::new();
                path_animation.push_back(target_position.v);            
                path_animation.push_back(entity_position.v);
                ev_animate.send(AnimateEvent { entity: event.entity, path: path_animation });
            }

        }
        ev_refresh_action.send(RefreshActionCostEvent);
    }
}


pub fn action_entity_get_hit(
    mut ev_gethit: EventReader<EntityGetHitEvent>,
    mut stats_health_q: Query<(&Stats, &mut Health, Option<&Player>)>,
    mut ev_die: EventWriter<EntityDeathEvent>,    
    mut ev_sound: EventWriter<SoundEvent>,
    mut ev_log: EventWriter<LogEvent>

) {
    for event in ev_gethit.read() {
        println!("Entity {:?} has been hit by {:?} for {:?} dmg.", event.entity, event.attacker, event.dmg);
        let Ok(defender_infos) = stats_health_q.get_mut(event.entity) else { 
            println!("Pas de stats / health pour le defender");
            continue };
        let (defender_stats, mut defender_health, _is_player) = defender_infos;

        // Roll resist.
        let dice_roll = roll_dices_against(defender_stats.resilience, 0);       // Pas d'opposant ni difficulté : On encaisse X dmg.
        let dmg = event.dmg.saturating_sub(dice_roll.success); 

        // Reducing health.
        defender_health.current = defender_health.current.saturating_sub(dmg);
        println!("Dmg on health for {:?} is now {:?}/{:?}", dmg, defender_health.current, defender_health.max);
        if defender_health.current == 0 {
            // SOUND
            ev_sound.send(SoundEvent{id:"death_scream".to_string()});
            ev_die.send(EntityDeathEvent { entity: event.entity });
            ev_log.send(LogEvent {entry: format!("{:?} has been killed by {:?}!", event.entity, event.attacker)});   // Log v0
        }
    }
}

pub fn entity_dies(
    mut commands: Commands,
    mut ev_die: EventReader<EntityDeathEvent>,    
    mut ev_refresh_action: EventWriter<RefreshActionCostEvent>,
    player_q: Query<&Player>,
    mut ev_message: EventWriter<MessageEvent>   //NEW MESSAGE EVENT SYSTEM v0.15.2
){
    for event in ev_die.read() {
        //TODO: Remove components, transform texture to Corpse.
        //commands.entity(event.entity).remove::<Stats>();
        //commands.entity(event.entity).remove::<Health>();
        //commands.entity(event.entity).remove::<Piece>();
        println!("Entity {:?} is dead", event.entity);
        if let Ok(_is_player) = player_q.get(event.entity) {  
            ev_message.send(MessageEvent(Box::new(GameOverMessage)));
        }
        commands.entity(event.entity).despawn();
        ev_refresh_action.send(RefreshActionCostEvent);
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

pub fn walk_combat_animation(    
    mut commands: Commands,
    mut ev_animate: EventReader<AnimateEvent>,
) {
    for ev in ev_animate.read() {
        let mut path = ev.path.clone();

        let mut path_animation: VecDeque<Vec3> = VecDeque::new();
        while !ev.path.is_empty() {
            let step = path.pop_front();
            let Some(current_step) = step else { break };
            let world_position = get_world_position(&current_step);        //TODO Est ce qu'un calcul de position Render doit etre là? Bof.
            let target = Vec3::new(world_position.0, world_position.1, 2.0);
            path_animation.push_back(target);
        }
        println!("PathAnimator created");
        commands.entity(ev.entity).insert(PathAnimator{path:VecDeque::from(path_animation), wait_anim: true});        
    }
}

#[derive(Resource)]
pub struct ActionInfos {
    pub cost: Option<u32>,
    pub path: Option<VecDeque<Vector2Int>>,
    pub target: Option<Vector2Int>,
    pub entity: Option<Entity>,
}

pub fn create_action_infos(
    query_character_turn: Query<(Entity, &ActionPoints, &BoardPosition), With<Player>>,
    query_occupied: Query<&BoardPosition, With<Occupier>>,
    board: Res<Map>,
    mut action_infos: ResMut<ActionInfos>,
    cursor: Res<Cursor>,
    piece_position: Query<&BoardPosition, (With<Health>, With<Stats>)>,
    mut ev_refresh_action: EventReader<RefreshActionCostEvent>,
) {
    for _event in ev_refresh_action.read() {
        //println!("Updating ActionInfos ");
        //Reset:
        action_infos.cost = None;
        action_infos.path = None;
        action_infos.target = None; //Some(cursor.grid_position);
        action_infos.entity = None;

        let Ok(player_infos) = query_character_turn.get_single() else { 
            println!("create action: No player infos");
            return };
        let (entity, action_points, position) = player_infos;
        action_infos.entity = Some(entity);

        let tile_position = cursor.grid_position;
        if !board.entity_tiles.contains_key(&tile_position) { 
            println!("Create action: out of map for {:?} with position: {:?}", entity, position);
            return }

        let mut has_target = false;
        if piece_position.iter().any(|board_position| board_position.v == tile_position) {
            has_target = true;
            action_infos.target = Some(tile_position);
        }
        //DEBUG: println!("creation action post has_target: has_target = {:?}, infos.target = {:?}", has_target, action_infos.target);

        let path_to_destination = find_path(
            position.v,
            tile_position,
            &board.entity_tiles.keys().cloned().collect(),
            &query_occupied.iter().map(|p| p.v).collect(),
            has_target,
        ); 

        let Some(path) = path_to_destination else { 
                //DEBUG: println!("Pas de Path");
            return };

        //DEBUG: println!("All return checks are done.");
        let mut ap_cost = path.len() as u32;
        if has_target {
            let ap_melee_cost = AP_COST_MELEE.saturating_sub(AP_COST_MOVE); // REMEMBER : En melee, le dernier pas est sur la cible donc il faut le retirer.
            ap_cost = ap_cost.saturating_add(ap_melee_cost)
        }

        if action_points.current >= ap_cost {
            action_infos.cost = Some(ap_cost);
            action_infos.path = Some(path);
        };

        // DEBUG: println!("Update action finale: cost: {:?}, path: {:?}, target: {:?}, entity: {:?}", action_infos.cost, action_infos.path, action_infos.target, action_infos.entity);
    }
}
