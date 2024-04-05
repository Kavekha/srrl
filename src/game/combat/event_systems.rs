use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{
    engine::{
        animations::events::{AnimateEvent, EffectEvent}, 
        asset_loaders::graphic_resources::GraphicsAssets, 
        audios::SoundEvent,
        render::get_world_position
    }, 
    game::{
        combat::{components::{AttackType, Die, GetHit, IsDead, MissHit, TryHit, WantToHit}, 
        rules::{roll_dices_against, DiceRollResult, AP_COST_MELEE, AP_COST_RANGED }}, 
        gamelog::LogEvent, 
        pieces::components::{Health, Occupier, Stats}, player::Player,        
        tileboard::components::BoardPosition, ui::ReloadUiEvent
    }, globals::ORDER_CORPSE, vectors::Vector2Int
};

use super::events::WantToHitEvent;
use super::{
    components::ActionPoints, events::{
        EntityEndTurnEvent, RefreshActionCostEvent, Turn
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

// 0.19b Ranged + Refacto.  // 0.19c TO CHANGE : Encore degueu car on a un Event qui vient du ranged... On s'en sort pas.
pub fn on_event_entity_want_hit(
    mut commands: Commands,
    mut ev_want_to_hit: EventReader<WantToHitEvent>,
){
    for event in ev_want_to_hit.read() {
        println!("Someone want to hit something.");
        let want_hit = WantToHit{ 
            mode: AttackType::RANGED,
            target: event.target
        };
        commands.entity(event.source).insert(want_hit);
    }
}

// 0.19d : utilisé par Ranged & Melee.
// Ici on verifie tout.
pub fn entity_want_hit(
    mut commands: Commands,
    want_hit_q: Query<(Entity, &WantToHit)>,
    player_q: Query<&Player>,    
    mut action_q: Query<&mut ActionPoints>,    
    mut ev_interface: EventWriter<ReloadUiEvent>,    
    mut ev_refresh_action: EventWriter<RefreshActionCostEvent>,    
    available_targets: Query<(Entity, &BoardPosition, &Stats), With<Health>>,
    stats_q: Query<&Stats>,        
    mut ev_log: EventWriter<LogEvent>,
) {
    for (entity, want) in want_hit_q.iter() {
        // Je le degage avant, car je sors à chaque cas non valide par la suite. Si c'est à la fin, je ne lirai pas cette commande.
        commands.entity(entity).remove::<WantToHit>();

        println!("RangedAttack: Refacto Combat 0.19b");
        println!("Je suis {:?} et j'attaque à la position {:?}", entity, want.target);

        let Ok(_attacker_stats) = stats_q.get(entity) else { 
            ev_log.send(LogEvent {entry: format!("ERROR: Not a valid fighter, can't attack. Stats missing.")});  // No Stats, can't be attacked.
            continue };    

        // Targets de la case:
        let target_entities = available_targets.iter().filter(|(_, position, _)| position.v == want.target).collect::<Vec<_>>(); 
        if target_entities.len() == 0 { 
            ev_log.send(LogEvent {entry: format!("There is no available target here.")});        // Log v0
            continue };     

        //Payer le prix de l'action.
        let Ok(mut action_points) = action_q.get_mut(entity) else { continue };
        match want.mode {
            AttackType::MELEE => consume_actionpoints(&mut action_points, AP_COST_MELEE),
            AttackType::RANGED => consume_actionpoints(&mut action_points, AP_COST_RANGED),
            //_ => println!("Want to Hit AP Cost non géré pour ce cas là.")
        };
            
        if let Ok(_is_player) = player_q.get(entity) {
            ev_interface.send(ReloadUiEvent); 
            ev_refresh_action.send(RefreshActionCostEvent);
        }

        for (target_entity, _target_position, _target_stats) in target_entities.iter() {     
            println!("Want hit: potentielle target: {:?}", *target_entity);
            // Can't hit yourself.
            if entity == * target_entity { 
                println!("On ne peut pas s'attaquer soit même.");
                continue; }; 

                let try_hit = TryHit { mode: want.mode.clone(), defender: *target_entity};       //TODO : A un moment, il faudra distinguer l'auteur de l'outil (source?).
                commands.entity(entity).insert(try_hit);     
        }
    }
}

// 0.19b
pub fn entity_try_hit(
    mut commands: Commands,
    try_hit_q: Query<(Entity, &TryHit)>,
    stats_q: Query<&Stats>,       
    //mut ev_gethit: EventWriter<EntityGetHitEvent>,
    mut ev_sound: EventWriter<SoundEvent>,
    mut ev_animate: EventWriter<AnimateEvent>,      
    position_q: Query<&BoardPosition>,   
){
    for (entity, attack) in try_hit_q.iter() {
        commands.entity(entity).remove::<TryHit>(); // On retire au debut, car command joué à la fin & si continue au milieu ne sera pas traité.
        println!("{:?} try to attack {:?}.", entity, attack.defender);
        //done.

        let Ok(attacker_stats) = stats_q.get(entity) else { 
            // DEBUG: println!("Pas de stats pour l'attaquant");
            continue };      
        let Ok(defender_stats) = stats_q.get(attack.defender) else { 
            // DEBUG: println!("Pas de stats pour l'attaquant");
            continue };     

        // Jet d'attaque. Tout ca est à mettre dans Rules.
        let dice_roll:DiceRollResult;
        let dmg:u32;
        match attack.mode {
            AttackType::MELEE => {
                dice_roll = roll_dices_against(attacker_stats.agility + attacker_stats.melee, defender_stats.logic + defender_stats.agility);   
                dmg = dice_roll.success.saturating_add(attacker_stats.strength as u32);
            },
            AttackType::RANGED => {
                dice_roll = roll_dices_against(attacker_stats.agility + attacker_stats.firearms, defender_stats.logic + defender_stats.agility);   
                dmg = dice_roll.success.saturating_add(attacker_stats.logic as u32);
            }
        }

        if dice_roll.success > 0 {
            commands.entity(attack.defender).insert(GetHit{ attacker: entity, mode: attack.mode.clone(), dmg: dmg});
            ev_sound.send(SoundEvent{id:"hit_punch_1".to_string()});
        } else {
            commands.entity(entity).insert(MissHit { mode: attack.mode.clone(), defender:attack.defender});
        }

        // Animation 
        let Ok(target_position) = position_q.get(attack.defender) else { continue };
        match attack.mode { 
            AttackType::MELEE => {
                if let Ok(entity_position) = position_q.get(entity) {
                    let mut path_animation: VecDeque<Vector2Int> = VecDeque::new();
                    path_animation.push_back(target_position.v);            
                    path_animation.push_back(entity_position.v);
                    ev_animate.send(AnimateEvent { entity: entity, path: path_animation });
                }
            },
            _ => println!("Want to hit, not supported yet for other than MELEE")
        };  
    }
}


// Refacto 0.19b
pub fn entity_miss_attack(
    mut commands: Commands,
    miss_hit_q: Query<(Entity, &MissHit)>,     
    mut ev_sound: EventWriter<SoundEvent>,    
    position_q: Query<&BoardPosition>,    
    mut ev_effect: EventWriter<EffectEvent>,
    name_q: Query<&Name>,
    mut ev_log: EventWriter<LogEvent>,
){
    for (entity, miss) in miss_hit_q.iter() {
        commands.entity(entity).remove::<MissHit>();

        ev_sound.send(SoundEvent{id:"hit_air_1".to_string()});
        if let Ok(position) = position_q.get(miss.defender) {
            let transform = get_world_position(&position.v);
            ev_effect.send(EffectEvent { id: "hit_punch_miss".to_string(), x: transform.0, y: transform.1 });
        };

        let Ok(entity_name) = name_q.get(entity) else { continue; };
        let Ok(defender_entity_name) = name_q.get(miss.defender) else { continue;};
        ev_log.send(LogEvent {entry: format!("{:?} misses {:?}!", entity_name, defender_entity_name)});        // Log v0
    }
}

// 0.19b
pub fn entity_get_hit(    
    mut commands: Commands,
    get_hit_q: Query<(Entity, &GetHit)>,     
    position_q: Query<&BoardPosition>,    
    mut ev_effect: EventWriter<EffectEvent>,
    name_q: Query<&Name>,
    mut ev_log: EventWriter<LogEvent>,    
    mut stats_health_q: Query<(&Stats, &mut Health, Option<&Player>)>,    
    //mut ev_die: EventWriter<EntityDeathEvent>,
){
    for (entity, get_hit) in get_hit_q.iter() {
        commands.entity(entity).remove::<GetHit>();

        let Ok(defender_infos) = stats_health_q.get_mut(entity) else { 
            println!("Pas de stats / health pour le defender");
            continue };
        let (defender_stats, mut defender_health, _is_player) = defender_infos;

        // Roll resist.
        let dice_roll = roll_dices_against(defender_stats.strength, 0);       // Pas d'opposant ni difficulté : On encaisse X dmg.
        let dmg = get_hit.dmg.saturating_sub(dice_roll.success); 

        // Reducing health.
        defender_health.current = defender_health.current.saturating_sub(dmg);
        println!("Dmg on health for {:?} is now {:?}/{:?}", dmg, defender_health.current, defender_health.max);
        if defender_health.current == 0 {            
            //ev_die.send(EntityDeathEvent { entity: entity, attacker: get_hit.attacker });
            commands.entity(entity).insert(Die { killer: get_hit.attacker});
        }
        // effect
        if let Ok(position) = position_q.get(entity) {
            let transform = get_world_position(&position.v);
            ev_effect.send(EffectEvent { id: "hit_punch_blood".to_string(), x: transform.0, y: transform.1 });
        };        
        //logs 
        let Ok(entity_name) = name_q.get(entity) else { continue; };
        let Ok(attacker_entity_name) = name_q.get(get_hit.attacker) else { continue;};
        if dice_roll.success == 0 {     // No dmg reduction.
            ev_log.send(LogEvent {entry: format!("{} takes a full blow from {}, for {:?} damages!", entity_name, attacker_entity_name, dmg)});        // Log v0
        }
        else if dmg > 0 {
            ev_log.send(LogEvent {entry: format!("{:?} hit {:?} for {:?} damages.", attacker_entity_name, entity_name, dmg)});        // Log v0
        } else {
            ev_log.send(LogEvent {entry: format!("{} takes a hit without effect from {}.",entity_name, attacker_entity_name)});        // Log v0
        }

    }
}


pub fn entity_dies(
    mut commands: Commands,    
    die_q: Query<(Entity, &Die)>,   
    mut body_q: Query<&mut Handle<Image>>,
    graph_assets: Res<GraphicsAssets>,    
    mut transform_q: Query<&mut Transform>,
    mut ev_refresh_action: EventWriter<RefreshActionCostEvent>,
    mut ev_sound: EventWriter<SoundEvent>,
    mut ev_log: EventWriter<LogEvent>,
    name_q: Query<&Name>,
){
    for (entity, death) in die_q.iter() {
        commands.entity(entity).remove::<Die>();

        println!("Entity {:?} is dead", entity);
        commands.entity(entity).insert(IsDead);
        commands.entity(entity).remove::<ActionPoints>();
        commands.entity(entity).remove::<Occupier>();

        // Transformation en Corps.
        if let Ok(mut body) = body_q.get_mut(entity) {
            *body = graph_assets.textures["blood"].clone();
        };
        if let Ok(mut transform) = transform_q.get_mut(entity) {
            transform.translation.z = ORDER_CORPSE;
        }
        // SOUND
        ev_sound.send(SoundEvent{id:"death_scream".to_string()});

        ev_refresh_action.send(RefreshActionCostEvent);

        //Logs.. TODO : Ameliorer.
        let Ok(entity_name) = name_q.get(entity) else { continue; };
        let Ok(attacker_entity_name) = name_q.get(death.killer) else { continue;};        
        ev_log.send(LogEvent {entry: format!("{:?} has been killed by {:?}!", entity_name, attacker_entity_name)});   // Log v0
    }
}


