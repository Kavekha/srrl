use std::collections::VecDeque;

use bevy::prelude::*;

use crate::engine::render::get_world_position;
use crate::{
    engine::{animations::events::{AnimateEvent, EffectEvent}, asset_loaders::graphic_resources::GraphicsAssets, audios::SoundEvent}, game::{
        combat::{components::IsDead, rules::roll_dices_against, AP_COST_MELEE, AP_COST_MOVE}, 
        gamelog::LogEvent, 
        pieces::components::{Health, Occupier, Stats}, player::{Cursor, Player},        
        tileboard::components::BoardPosition, ui::ReloadUiEvent
    }, globals::ORDER_CORPSE, map_builders::map::Map, vectors::{find_path, Vector2Int}
};

use super::events::EntityHitMissEvent;
use super::{
    components::ActionPoints, events::{
        EntityDeathEvent, EntityEndTurnEvent, EntityGetHitEvent, EntityHitTryEvent, RefreshActionCostEvent, Turn
    }, rules::consume_actionpoints
};


/// Gestion de l'action de forfeit.
pub fn action_entity_end_turn(
    mut ev_endturn: EventReader<EntityEndTurnEvent>,
    mut query_character_turn: Query<(Entity, &mut ActionPoints, Option<&Player>), With<Turn>>,
    mut ev_interface: EventWriter<ReloadUiEvent>,  
    mut ev_refresh_action: EventWriter<RefreshActionCostEvent>,
    mut ev_log: EventWriter<LogEvent>
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
            ev_log.send(LogEvent{entry:format!("You forfeit your turn.")});  //LOG
        }
    }
}


// TODO : Au choix: Try_attack devrait soit verifier que l'entité peut accomplir son attaque, soit tester la réussite de son attaque.
pub fn action_entity_try_attack(
    mut ev_try_attack: EventReader<EntityHitTryEvent>,    
    mut ev_gethit: EventWriter<EntityGetHitEvent>,
    mut action_q: Query<&mut ActionPoints>,
    position_q: Query<&BoardPosition>,
    player_q: Query<&Player>,
    //available_targets: Query<(Entity, &BoardPosition, &Stats), With<Health>>,
    available_targets: Query<(Entity, &BoardPosition, &Stats), With<Health>>,
    stats_q: Query<&Stats>,
    mut ev_interface: EventWriter<ReloadUiEvent>,    
    mut ev_refresh_action: EventWriter<RefreshActionCostEvent>,
    mut ev_animate: EventWriter<AnimateEvent>,
    mut ev_sound: EventWriter<SoundEvent>,
    mut ev_try_miss: EventWriter<EntityHitMissEvent>
){
    for event in ev_try_attack.read() {
        // Verification. Devrait être ailleurs.
        println!("Je suis {:?} et j'attaque à la position {:?}", event.entity, event.target);

        // TODO : maybe refresh in consume? Maybe consume in some Event?
        let Ok(mut action_points) = action_q.get_mut(event.entity) else { continue };
        consume_actionpoints(&mut action_points, AP_COST_MELEE);
        if let Ok(_is_player) = player_q.get(event.entity) {
            ev_interface.send(ReloadUiEvent);   // Utile? TOCHECK
            ev_refresh_action.send(RefreshActionCostEvent); // Ui ? TOCHECK
        }

        // Targets de la case:
        let target_entities = available_targets.iter().filter(|(_, position, _)| position.v == event.target).collect::<Vec<_>>(); 
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
                continue; }; 

            // Animation.
            if let Ok(entity_position) = position_q.get(event.entity) {
                let mut path_animation: VecDeque<Vector2Int> = VecDeque::new();
                path_animation.push_back(target_position.v);            
                path_animation.push_back(entity_position.v);
                ev_animate.send(AnimateEvent { entity: event.entity, path: path_animation });
            }

            // L'attaque est tenté contre toutes les personnes de la cellule.
            let dice_roll = roll_dices_against(attacker_stats.attack, target_stats.dodge);   
            let dmg = dice_roll.success.saturating_add(attacker_stats.power as u32);

            if dice_roll.success > 0 {
                // DEBUG: println!("HIT target with {:?} success! for {:?} dmg", dice_roll.success, dmg);
                ev_gethit.send(EntityGetHitEvent { entity: * target_entity, attacker: event.entity, dmg: dmg });
                ev_sound.send(SoundEvent{id:"hit_punch_1".to_string()});
            } else {
                ev_try_miss.send(EntityHitMissEvent{entity: event.entity, defender: *target_entity});
                continue;
            }
        }
    }
}

pub fn action_entity_miss_attack(
    mut ev_try_miss: EventReader<EntityHitMissEvent>,
    name_q: Query<&Name>,
    mut ev_sound: EventWriter<SoundEvent>,
    mut ev_log: EventWriter<LogEvent>,
    mut ev_effect: EventWriter<EffectEvent>,
    position_q: Query<&BoardPosition>,
){
    for event in ev_try_miss.read() {
        ev_sound.send(SoundEvent{id:"hit_air_1".to_string()});
        // effect
        if let Ok(position) = position_q.get(event.defender) {
            let transform = get_world_position(&position.v);
            ev_effect.send(EffectEvent { id: "hit_punch_miss".to_string(), x: transform.0, y: transform.1 });
        }; 
        //logs 
        let Ok(entity_name) = name_q.get(event.entity) else { continue; };
        let Ok(defender_entity_name) = name_q.get(event.defender) else { continue;};
        ev_log.send(LogEvent {entry: format!("{:?} try to hit {:?} but misses.", entity_name, defender_entity_name)});        // Log v0
    }
}


pub fn action_entity_get_hit(
    mut ev_gethit: EventReader<EntityGetHitEvent>,
    mut stats_health_q: Query<(&Stats, &mut Health, Option<&Player>)>,
    mut ev_die: EventWriter<EntityDeathEvent>,    
    mut ev_log: EventWriter<LogEvent>,
    name_q: Query<&Name>,
    mut ev_effect: EventWriter<EffectEvent>,
    position_q: Query<&BoardPosition>, 
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
            ev_die.send(EntityDeathEvent { entity: event.entity, attacker: event.attacker });
        }
        // effect
        if let Ok(position) = position_q.get(event.entity) {
            let transform = get_world_position(&position.v);
            ev_effect.send(EffectEvent { id: "hit_punch_blood".to_string(), x: transform.0, y: transform.1 });
        };        
        //logs 
        let Ok(entity_name) = name_q.get(event.entity) else { continue; };
        let Ok(attacker_entity_name) = name_q.get(event.attacker) else { continue;};
        if dice_roll.success == 0 {     // No dmg reduction.
            ev_log.send(LogEvent {entry: format!("{} takes a full beatdown by {}, for {:?} damages!", entity_name, attacker_entity_name, dmg)});        // Log v0
        }
        else if dmg > 0 {
            ev_log.send(LogEvent {entry: format!("{:?} hit {:?} for {:?} damages.", attacker_entity_name, entity_name, dmg)});        // Log v0
        } else {
            ev_log.send(LogEvent {entry: format!("{} takes a blow without effect from {}.",entity_name, attacker_entity_name)});        // Log v0
        }
    }
}

//To treat at end of turn?
pub fn entity_dies(
    mut commands: Commands,
    mut ev_die: EventReader<EntityDeathEvent>,    
    mut ev_refresh_action: EventWriter<RefreshActionCostEvent>,
    name_q: Query<&Name>,
    graph_assets: Res<GraphicsAssets>,
    mut ev_sound: EventWriter<SoundEvent>,
    mut ev_log: EventWriter<LogEvent>,
    mut body_q: Query<&mut Handle<Image>>,
    mut transform_q: Query<&mut Transform>

){
    for event in ev_die.read() {
        println!("Entity {:?} is dead", event.entity);
        commands.entity(event.entity).insert(IsDead);
        commands.entity(event.entity).remove::<ActionPoints>();
        commands.entity(event.entity).remove::<Occupier>();

        // Transformation en Corps.
        if let Ok(mut body) = body_q.get_mut(event.entity) {
            *body = graph_assets.textures["blood"].clone();
        };
        if let Ok(mut transform) = transform_q.get_mut(event.entity) {
            transform.translation.z = ORDER_CORPSE;
        }
        // SOUND
        ev_sound.send(SoundEvent{id:"death_scream".to_string()});

        ev_refresh_action.send(RefreshActionCostEvent);

        //Logs.. TODO : Ameliorer.
        let Ok(entity_name) = name_q.get(event.entity) else { continue; };
        let Ok(attacker_entity_name) = name_q.get(event.attacker) else { continue;};        
        ev_log.send(LogEvent {entry: format!("{:?} has been killed by {:?}!", entity_name, attacker_entity_name)});   // Log v0
    }
}


// Ui?

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
    piece_position: Query<&BoardPosition, (With<Health>, With<Stats>, Without<IsDead>)>,
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
            //println!("Create action: out of map for {:?} with position: {:?}", entity, position);
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
