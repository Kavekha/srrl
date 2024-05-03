use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{
    engine::{animations::events::AnimateEvent, 
    asset_loaders::GraphicsAssets, audios::SoundEvent}, 
    game::{combat::{combat_system::components::{GetHit, MissHit}, events::{RefreshActionCostEvent, Turn, WantToHitEvent}}, commons::is_in_sight, effects::{add_effect, components::{EffectType, Targets}}, game_generation::character_creation::components::{Attributes, Health, Occupier, Skills}, gamelog::LogEvent, player::Player, rules::{combat_test, consume_actionpoints, dmg_resist_test, enough_ap_for_action, RuleCombatResult, AP_COST_MELEE, AP_COST_RANGED, RANGED_ATTACK_RANGE_MAX}, tileboard::components::BoardPosition, ui::events::ReloadUiEvent},
    globals::ORDER_CORPSE, map_builders::map::Map, vectors::Vector2Int};

use super::components::{ActionPoints, AttackType, Die, IsDead, TryHit, WantToForfeit, WantToHit};




// 0.19b Ranged + Refacto.  // 0.19c TOCHANGE : Encore degueu car on a un Event qui vient du ranged... On s'en sort pas.
pub fn on_event_entity_want_hit(
    mut commands: Commands,
    mut ev_want_to_hit: EventReader<WantToHitEvent>,
){
    for event in ev_want_to_hit.read() {
        //println!("Someone want to hit something.");
        let want_hit = WantToHit{ 
            mode: AttackType::RANGED,
            target: event.target
        };
        commands.entity(event.source).insert(want_hit);
    }
}


pub fn entity_want_forfeit(
    mut commands: Commands,
    mut entity_actions_q: Query<(Entity, &mut ActionPoints, Option<&Player>), (With<Turn>, With<WantToForfeit>)>,
    mut ev_interface: EventWriter<ReloadUiEvent>,  
    mut ev_refresh_action: EventWriter<RefreshActionCostEvent>,
    mut ev_log: EventWriter<LogEvent>
) {
    let mut to_remove = Vec::new();
    for (entity, mut action_points, is_player) in entity_actions_q.iter_mut() {
        let lost_value = action_points.max.saturating_add(0);
        consume_actionpoints(&mut action_points, lost_value);
        to_remove.push(entity);
        
        if is_player.is_some() {
            ev_interface.send(ReloadUiEvent);
            ev_refresh_action.send(RefreshActionCostEvent);
            ev_log.send(LogEvent{entry:format!("You forfeit your turn.")});  //LOG
        }
    }
    for entity in to_remove {
        commands.entity(entity).remove::<WantToForfeit>();
    }
}

// 0.19d : utilisé par Ranged & Melee.
// Ici on verifie tout.
// 0.20a : Review Query OK
pub fn entity_want_hit(
    mut commands: Commands,
    mut want_hit_q: Query<(Entity, &WantToHit, &mut ActionPoints, &BoardPosition, Option<&Player>), (With<Attributes>, Without<IsDead>)>,
    //player_q: Query<&Player>,    
    //mut action_q: Query<&mut ActionPoints>,    
    mut ev_interface: EventWriter<ReloadUiEvent>,    
    mut ev_refresh_action: EventWriter<RefreshActionCostEvent>,    
    available_targets: Query<(Entity, &BoardPosition, &Attributes), (With<Health>, Without<IsDead>)>,
    //position_q: Query<&BoardPosition>,
    //stats_q: Query<&Stats>,        
    mut ev_log: EventWriter<LogEvent>,
    board: Res<Map>,
) {
    let mut to_remove = Vec::new();
    for (entity, want, mut action_points, entity_position, is_player) in want_hit_q.iter_mut() {
        // Je le degage avant, car je sors à chaque cas non valide par la suite. Si c'est à la fin, je ne lirai pas cette commande.
        to_remove.push(entity);

        // Verifie si assez de AP pour l'action.
        let Ok(_) = enough_ap_for_action(&action_points, &want.mode) else { 
            if is_player.is_some() {
                ev_log.send(LogEvent {entry: format!("Not enough AP for this action.")});  // No Stats, can't be attacked.
            }
            continue };
        //println!("Je suis {:?} et j'attaque à la position {:?}", entity, want.target);

        // Targets de la case:
        let target_entities = available_targets.iter().filter(|(_, position, _)| position.v == want.target).collect::<Vec<_>>(); 
        if target_entities.len() == 0 { 
            if is_player.is_some() {
                ev_log.send(LogEvent {entry: format!("There is no available target here.")});        // Log v0
            }
            continue };     

        // Taper!
        let mut could_hit_someone= false;
        for (target_entity, target_position, _target_stats) in target_entities.iter() {     
            //println!("Want hit: potentielle target: {:?}", *target_entity);
            // Can't hit yourself.
            if entity == * target_entity { 
                //println!("On ne peut pas s'attaquer soit même.");
                continue; }; 

            // 0.19e : Visuel : Ne prends pas en compte le type. TODO: Reach lié à l'attaque / equipement.
            let Ok(_in_los) = is_in_sight(&board, &entity_position.v, &target_position.v, RANGED_ATTACK_RANGE_MAX) else {
                if is_player.is_some() {
                    ev_log.send(LogEvent {entry: format!("Target is not in view.")}); 
                }
                continue;
            };

            could_hit_someone= true;
            let try_hit = TryHit { mode: want.mode.clone(), defender: *target_entity};       //TODO : A un moment, il faudra distinguer l'auteur de l'outil (source?).
            commands.entity(entity).insert(try_hit);     
        }

        //Payer le prix de l'action.
        if could_hit_someone {                
            match want.mode {
                AttackType::MELEE => consume_actionpoints(&mut action_points, AP_COST_MELEE),
                AttackType::RANGED => consume_actionpoints(&mut action_points, AP_COST_RANGED),
                //_ => println!("Want to Hit AP Cost non géré pour ce cas là.")
            };
            if is_player.is_some() {
                ev_interface.send(ReloadUiEvent); 
                ev_refresh_action.send(RefreshActionCostEvent);
            }
        }
    }
    for entity in to_remove {
        commands.entity(entity).remove::<WantToHit>();
    }
}

// 0.19b
pub fn entity_try_hit(
    mut commands: Commands,
    try_hit_q: Query<(Entity, &TryHit), Without<IsDead>>,
    attributes_n_skills_q: Query<(&Attributes, Option<&Skills>)>,       
    //mut ev_gethit: EventWriter<EntityGetHitEvent>,
    mut ev_sound: EventWriter<SoundEvent>,
    mut ev_animate: EventWriter<AnimateEvent>,      
    position_q: Query<&BoardPosition>,   
){
    let mut to_remove = Vec::new();
    for (entity, attack) in try_hit_q.iter() {
        to_remove.push(entity);
        println!("{:?} try to attack {:?}.", entity, attack.defender);
        //done.
  
        let Ok(attacker_infos) = attributes_n_skills_q.get(entity) else { 
            // DEBUG: println!("Pas de stats pour l'attaquant");
            continue };   
        let Ok(defender_infos) = attributes_n_skills_q.get(attack.defender) else { 
            // DEBUG: println!("Pas de stats pour le defender");
            continue };     

        // Jet d'attaque. Tout ca est à mettre dans Rules.
        let combat_result: RuleCombatResult;
        //let dice_roll:DiceRollResult;
        //let dmg:u32;
        match attack.mode {
            AttackType::MELEE => {
                combat_result = combat_test(&AttackType::MELEE, attacker_infos, defender_infos);
            },
            AttackType::RANGED => {
                combat_result = combat_test(&AttackType::RANGED, attacker_infos, defender_infos);
            }
        }

        if combat_result.success {
            commands.entity(attack.defender).insert(GetHit{ attacker: entity, mode: attack.mode.clone(), dmg: combat_result.dmg});
            match attack.mode {
                AttackType::MELEE => {
                    ev_sound.send(SoundEvent{id:"hit_punch_1".to_string()});
                },
                AttackType::RANGED => {
                    ev_sound.send(SoundEvent{id:"gun_shot_1".to_string()});
                    ev_sound.send(SoundEvent{id:"gun_reload_1".to_string()});           
                }
            }            
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
                    ev_animate.send(AnimateEvent { entity: entity, path: path_animation, wait_anim: true });
                }
            },
            AttackType::RANGED => { 
                add_effect(None, EffectType::Particle { id: "hit_muzzle_1".to_string(), duration: 1.0 }, Targets::Single{ target:entity });
            },
        };  
    }
    for entity in to_remove {
        commands.entity(entity).remove::<TryHit>(); // On retire au debut, car command joué à la fin & si continue au milieu ne sera pas traité.
    }
}


// Refacto 0.19b
pub fn entity_miss_attack(
    mut commands: Commands,
    miss_hit_q: Query<(Entity, &MissHit), Without<IsDead>>,     
    name_q: Query<&Name>,
    mut ev_sound: EventWriter<SoundEvent>,    
    mut ev_log: EventWriter<LogEvent>,
){
    let mut to_remove = Vec::new();
    for (entity, miss) in miss_hit_q.iter() {
        to_remove.push(entity);        
        // sounds.
        match miss.mode {
            AttackType::MELEE => {
                ev_sound.send(SoundEvent{id:"hit_air_1".to_string()});
            },
            AttackType::RANGED => {
                ev_sound.send(SoundEvent{id:"gun_shot_1".to_string()});
                ev_sound.send(SoundEvent{id:"gun_reload_1".to_string()});           
            }
        }    

        add_effect(None, EffectType::Particle { id: "hit_punch_miss".to_string(), duration: 1.0 }, Targets::Single{ target:miss.defender });

        let Ok(entity_name) = name_q.get(entity) else { continue; };
        let Ok(defender_entity_name) = name_q.get(miss.defender) else { continue;};
        ev_log.send(LogEvent {entry: format!("{:?} misses {:?}!", entity_name, defender_entity_name)});        // Log v0
    }
    for entity in to_remove {
        commands.entity(entity).remove::<MissHit>();
    }
}

// 0.19b
pub fn entity_get_hit(    
    mut commands: Commands,
    get_hit_q: Query<(Entity, &GetHit), Without<IsDead>>,     
    name_q: Query<&Name>,     
    mut stats_health_q: Query<(&Attributes, &mut Health, Option<&Player>)>, 
    mut ev_log: EventWriter<LogEvent>,    
){
    let mut to_remove = Vec::new();
    for (entity, get_hit) in get_hit_q.iter() {
        to_remove.push(entity);

        let Ok(defender_infos) = stats_health_q.get_mut(entity) else { 
            //println!("Pas de stats / health pour le defender");
            continue };
        let (defender_stats, defender_health, _is_player) = defender_infos;

        // Roll resist.
        let test_resist = dmg_resist_test(&get_hit.mode, &defender_stats);
        let final_dmg = get_hit.dmg.saturating_sub(test_resist.dmg_reduction) as u32; 

        // Reducing health. Effect in 0.20g
        add_effect(
            Some(get_hit.attacker),
            EffectType::Damage{ amount: final_dmg as i32 },
            Targets::Single{ target: entity }
        );
        if defender_health.current <= 0 {            
            commands.entity(entity).insert(Die { killer: get_hit.attacker});
        }              
        //logs 
        let Ok(entity_name) = name_q.get(entity) else { continue; };
        let Ok(attacker_entity_name) = name_q.get(get_hit.attacker) else { continue;};
        if test_resist.success == false {     // No dmg reduction.
            ev_log.send(LogEvent {entry: format!("{} takes a full blow from {}, for {:?} damages!", entity_name, attacker_entity_name, final_dmg)});        // Log v0
        }
        else if final_dmg > 0 {
            ev_log.send(LogEvent {entry: format!("{:?} hit {:?} for {:?} damages.", attacker_entity_name, entity_name, final_dmg)});        // Log v0
        } else {
            ev_log.send(LogEvent {entry: format!("{} takes a hit without effect from {}.",entity_name, attacker_entity_name)});        // Log v0
        }
    }
    for entity in to_remove {        
        commands.entity(entity).remove::<GetHit>();
    }
}


pub fn entity_dies(
    mut commands: Commands,    
    mut die_q: Query<(Entity, &Die, &mut Transform)>,   
    mut body_q: Query<&mut Handle<Image>>,
    graph_assets: Res<GraphicsAssets>,    
    //mut transform_q: Query<&mut Transform>,
    mut ev_refresh_action: EventWriter<RefreshActionCostEvent>,
    mut ev_sound: EventWriter<SoundEvent>,
    mut ev_log: EventWriter<LogEvent>,
    name_q: Query<&Name>,
){
    let mut to_remove=Vec::new();
    for (entity, death, mut transform) in die_q.iter_mut() {
        to_remove.push(entity);        

        //println!("Entity {:?} is dead", entity);
        commands.entity(entity).insert(IsDead);

        // Transformation en Corps.        
        if let Ok(mut body) = body_q.get_mut(entity) {
            *body = graph_assets.textures["blood"].clone();
        };
        //if let Ok(mut transform) = transform_q.get_mut(entity) {
            transform.translation.z = ORDER_CORPSE;
        //}
        // SOUND
        ev_sound.send(SoundEvent{id:"death_scream".to_string()});

        ev_refresh_action.send(RefreshActionCostEvent);

        //Logs.. 
        let Ok(entity_name) = name_q.get(entity) else { continue; };
        let Ok(attacker_entity_name) = name_q.get(death.killer) else { continue;};        
        ev_log.send(LogEvent {entry: format!("{:?} has been killed by {:?}!", entity_name, attacker_entity_name)});   // Log v0
    }
    for entity in to_remove {
        commands.entity(entity).remove::<Die>();
        commands.entity(entity).remove::<ActionPoints>();
        commands.entity(entity).remove::<Occupier>();
    }
}


